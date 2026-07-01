use std::collections::HashMap;
use crate::ir::{Function, Stmt, Operand};

#[derive(Debug, Clone)]
pub struct ResolvedCall {
    pub block_id: usize,
    pub target: String,
    pub args: Vec<ResolvedArg>,
}

#[derive(Debug, Clone)]
pub enum ResolvedArg {
    Const(i64),
    StringLiteral(String),
    CallResult(String),
    FieldLoad { base_call: String, offset: i64 },
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

fn build_def_map(func: &Function) -> HashMap<usize, Stmt> {
    let mut def_map: HashMap<usize, Stmt> = HashMap::new();
    for block in &func.blocks {
        for stmt in &block.stmts {
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
                    let left_res = trace_operand(left, def_map, rodata, depth + 1);
                    let right_res = trace_operand(right, def_map, rodata, depth + 1);
                    match (&left_res, &right_res) {
                        (ResolvedArg::Const(0), _) => right_res,
                        (_, ResolvedArg::Const(0)) => left_res,
                        _ => ResolvedArg::Unknown(format!("{:?} op {:?}", left_res, right_res)),
                    }
                }

                Some(Stmt::Phi { choices, .. }) => {
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


pub struct StructPattern {
    pub base_var: usize,
    pub offsets: Vec<i64>,
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
        .filter(|(_, offsets)| offsets.len() >= 2)
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
