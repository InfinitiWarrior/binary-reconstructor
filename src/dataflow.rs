/// Dataflow analysis pass
///
/// Input: a lifted Function (SSA IR with explicit basic blocks)
/// Output: resolved call sites with concrete argument information
///
/// This is the core of semantic extraction. For each Call node in the IR,
/// we walk backwards through use-def chains to find what values actually
/// flow into the argument registers at the call site.
///
/// What we can resolve here:
///   - String literal arguments (arg = address in .rodata)
///   - Integer constants (arg = immediate value)
///   - Return values from prior calls (arg = result of malloc, fopen, etc.)
///   - Struct field loads (arg = load from known base + offset)
///
/// What we can't resolve without symbolic execution:
///   - Values computed through complex arithmetic
///   - Values read from stdin/environment at runtime
///   - Pointer targets that depend on runtime heap layout

use std::collections::HashMap;
use crate::ir::{Function, Stmt, Operand};
/// A resolved call site: we know who is being called and what the arguments
/// concretely are (as best as static analysis can determine).
#[derive(Debug, Clone)]
pub struct ResolvedCall {
    pub block_id: usize,
    pub target: String,
    /// Each argument: either resolved to a concrete value or left as a Var reference.
    pub args: Vec<ResolvedArg>,
}

#[derive(Debug, Clone)]
pub enum ResolvedArg {
    /// A known integer constant.
    Const(i64),
    /// A string from the binary's read-only data section.
    StringLiteral(String),
    /// The return value of a prior call (e.g., malloc return used as buffer).
    CallResult(String),
    /// A load from a struct at a known offset (struct field access pattern).
    FieldLoad { base_call: String, offset: i64 },
    /// Unresolved: we traced it but couldn't recover concrete meaning.
    Unknown(String),
}

impl std::fmt::Display for ResolvedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolvedArg::Const(n) => write!(f, "{}", n),
            ResolvedArg::StringLiteral(s) => write!(f, "\"{}\"", s),
            ResolvedArg::CallResult(name) => write!(f, "<result of {}>", name),
            ResolvedArg::FieldLoad { base_call, offset } =>
                write!(f, "<field+{:#x} of {}>", offset, base_call),
            ResolvedArg::Unknown(desc) => write!(f, "<unknown: {}>", desc),
        }
    }
}

/// Build a map from Var -> the Stmt that defined it, across all blocks.
/// In proper SSA this is trivial (one definition per variable).
fn build_def_map(func: &Function) -> HashMap<usize, Stmt> {
    let mut def_map: HashMap<usize, Stmt> = HashMap::new();
    for block in &func.blocks {
        for stmt in &block.stmts {
            // Extract the defined variable from each statement type.
            let defined_var = match stmt {
                Stmt::BinOp { dest, .. }      => Some(dest.0),
                Stmt::Load { dest, .. }        => Some(dest.0),
                Stmt::Call { dest: Some(d), .. } => Some(d.0),
                Stmt::IndirectCall { dest: Some(d), .. } => Some(d.0),
                Stmt::Phi { dest, .. }         => Some(dest.0),
                Stmt::Assign { dest, .. }      => Some(dest.0),
                _ => None,
            };
            if let Some(var_id) = defined_var {
                def_map.insert(var_id, stmt.clone());
            }
        }
    }
    def_map
}

/// Trace an operand backwards through use-def chains to find its concrete value.
/// `rodata` maps virtual addresses to string content (extracted from the binary's .rodata section).
/// `depth` limits recursion to prevent cycles (shouldn't exist in SSA, but be safe).
fn trace_operand(
    op: &Operand,
    def_map: &HashMap<usize, Stmt>,
    rodata: &HashMap<u64, String>,
    depth: u8,
) -> ResolvedArg {
    if depth > 8 {
        return ResolvedArg::Unknown("max trace depth".to_string());
    }

    match op {
        Operand::Const(n) => ResolvedArg::Const(*n),

        Operand::Symbol(name, offset) => {
            // If the symbol is in rodata, it's a string literal.
            // (In practice, symbol addresses are resolved by the ELF parser;
            // here we treat the name itself as the key.)
            ResolvedArg::StringLiteral(name.clone())
        }

        Operand::Var(v) => {
            match def_map.get(&v.0) {
                None => ResolvedArg::Unknown(format!("v{} undefined", v.0)),

                Some(Stmt::Assign { src, .. }) => {
                    trace_operand(src, def_map, rodata, depth + 1)
                }

                Some(Stmt::Call { target, .. }) => {
                    ResolvedArg::CallResult(target.clone())
                }

                Some(Stmt::Load { base, offset, .. }) => {
                    // A field load. Trace the base to find the struct origin.
                    let base_resolved = trace_operand(base, def_map, rodata, depth + 1);
                    match base_resolved {
                        ResolvedArg::CallResult(name) =>
                            ResolvedArg::FieldLoad { base_call: name, offset: *offset },
                        _ =>
                            ResolvedArg::Unknown(format!("load+{:#x}", offset)),
                    }
                }

                Some(Stmt::BinOp { op: binop, left, right, .. }) => {
                    // If it's an add with a constant (address + offset pattern), trace through.
                    let left_res = trace_operand(left, def_map, rodata, depth + 1);
                    let right_res = trace_operand(right, def_map, rodata, depth + 1);
                    // Simplification: if one side is a const 0, return the other.
                    match (&left_res, &right_res) {
                        (ResolvedArg::Const(0), _) => right_res,
                        (_, ResolvedArg::Const(0)) => left_res,
                        _ => ResolvedArg::Unknown(format!("{:?} op {:?}", left_res, right_res)),
                    }
                }

                Some(Stmt::Phi { choices, .. }) => {
                    // At a join point, we can only say "one of these values".
                    // For now, trace the first choice and note it's a phi.
                    if let Some((_, op)) = choices.first() {
                        trace_operand(op, def_map, rodata, depth + 1)
                    } else {
                        ResolvedArg::Unknown("empty phi".to_string())
                    }
                }

                _ => ResolvedArg::Unknown(format!("v{}", v.0)),
            }
        }
    }
}

/// Main entry point: resolve all call sites in a function.
pub fn resolve_calls(
    func: &Function,
    rodata: &HashMap<u64, String>,
) -> Vec<ResolvedCall> {
    let def_map = build_def_map(func);
    let mut resolved = vec![];

    for block in &func.blocks {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Call { target, args, .. } => {
                    let resolved_args: Vec<ResolvedArg> = args.iter()
                        .map(|a| trace_operand(a, &def_map, rodata, 0))
                        .collect();
                    resolved.push(ResolvedCall {
                        block_id: block.id,
                        target: target.clone(),
                        args: resolved_args,
                    });
                }
                Stmt::IndirectCall { ptr, args, .. } => {
                    let ptr_resolved = trace_operand(ptr, &def_map, rodata, 0);
                    let resolved_args: Vec<ResolvedArg> = args.iter()
                        .map(|a| trace_operand(a, &def_map, rodata, 0))
                        .collect();
                    resolved.push(ResolvedCall {
                        block_id: block.id,
                        target: format!("*{}", ptr_resolved),
                        args: resolved_args,
                    });
                }
                _ => {}
            }
        }
    }

    resolved
}

/// Identify struct access patterns: groups of Load statements that share the
/// same base Var but different offsets. Each group suggests a struct type.
pub struct StructPattern {
    /// The Var holding the base pointer.
    pub base_var: usize,
    /// Offsets accessed on this base.
    pub offsets: Vec<i64>,
    /// If the base came from a call, this is the callee name.
    pub origin_call: Option<String>,
}

pub fn find_struct_patterns(func: &Function) -> Vec<StructPattern> {
    let def_map = build_def_map(func);
    let mut base_to_offsets: HashMap<usize, Vec<i64>> = HashMap::new();

    for block in &func.blocks {
        for stmt in &block.stmts {
            if let Stmt::Load { base: Operand::Var(v), offset, .. } = stmt {
                base_to_offsets.entry(v.0).or_default().push(*offset);
            }
            if let Stmt::Store { base: Operand::Var(v), offset, .. } = stmt {
                base_to_offsets.entry(v.0).or_default().push(*offset);
            }
        }
    }

    base_to_offsets.into_iter()
        .filter(|(_, offsets)| offsets.len() >= 2) // at least 2 field accesses to suggest a struct
        .map(|(var_id, mut offsets)| {
            offsets.sort_unstable();
            offsets.dedup();
            let origin_call = def_map.get(&var_id).and_then(|stmt| {
                if let Stmt::Call { target, .. } = stmt { Some(target.clone()) } else { None }
            });
            StructPattern { base_var: var_id, offsets, origin_call }
        })
        .collect()
}