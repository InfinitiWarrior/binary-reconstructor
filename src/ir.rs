/// Every value in the IR is a unique definition, identified by a variable index.
/// This is the foundation of SSA form: one definition per variable, ever.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var(pub usize);

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// The value of an operand at a given point: either a known constant, a variable
/// produced by an earlier definition, or a symbol (imported name + optional offset).
#[derive(Debug, Clone)]
pub enum Operand {
    Const(i64),
    Var(Var),
    Symbol(String, i64), // symbol name, addend
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::Const(n) => write!(f, "{:#x}", n),
            Operand::Var(v) => write!(f, "{}", v),
            Operand::Symbol(s, 0) => write!(f, "&{}", s),
            Operand::Symbol(s, off) => write!(f, "&{}+{:#x}", s, off),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, And, Or, Xor, Shl, Shr, Sar,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::And => write!(f, "&"),
            BinOp::Or  => write!(f, "|"),
            BinOp::Xor => write!(f, "^"),
            BinOp::Shl => write!(f, "<<"),
            BinOp::Shr => write!(f, ">>"),
            BinOp::Sar => write!(f, "a>>"),
        }
    }
}

/// A single IR statement. One of:
/// - A binary operation producing a new variable
/// - A load from memory
/// - A store to memory
/// - A direct call to a known symbol with explicit argument list
/// - An indirect call through a register (vtable dispatch etc.)
/// - A phi node (SSA join point at a CFG merge)
/// - A return
/// - A conditional branch
/// - An unconditional jump
#[derive(Debug, Clone)]
pub enum Stmt {
    /// dest = left op right
    BinOp { dest: Var, op: BinOp, left: Operand, right: Operand },

    /// dest = *base[offset]  (size in bytes)
    Load { dest: Var, base: Operand, offset: i64, size: u8 },

    /// *base[offset] = src
    Store { base: Operand, offset: i64, src: Operand, size: u8 },

    /// dest = target(args...)  where target is a known import/symbol
    Call { dest: Option<Var>, target: String, args: Vec<Operand> },

    /// dest = (*ptr)(args...)  where ptr is a runtime value
    IndirectCall { dest: Option<Var>, ptr: Operand, args: Vec<Operand> },

    /// SSA phi: dest = phi([(block_id, operand), ...])
    Phi { dest: Var, choices: Vec<(usize, Operand)> },

    /// return value (None = void)
    Return(Option<Operand>),

    /// if cond goto true_block else false_block
    Branch { cond: Operand, true_bb: usize, false_bb: usize },

    /// goto target
    Jump(usize),

    /// dest = operand  (copy / move)
    Assign { dest: Var, src: Operand },
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::BinOp { dest, op, left, right } =>
                write!(f, "  {} = {} {} {}", dest, left, op, right),
            Stmt::Load { dest, base, offset, size } =>
                write!(f, "  {} = load{}({} + {:#x})", dest, size*8, base, offset),
            Stmt::Store { base, offset, src, size } =>
                write!(f, "  store{}({} + {:#x}, {})", size*8, base, offset, src),
            Stmt::Call { dest, target, args } => {
                let arg_str = args.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(", ");
                match dest {
                    Some(d) => write!(f, "  {} = {}({})", d, target, arg_str),
                    None    => write!(f, "  {}({})", target, arg_str),
                }
            }
            Stmt::IndirectCall { dest, ptr, args } => {
                let arg_str = args.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(", ");
                match dest {
                    Some(d) => write!(f, "  {} = (*{})({})", d, ptr, arg_str),
                    None    => write!(f, "  (*{})({})", ptr, arg_str),
                }
            }
            Stmt::Phi { dest, choices } => {
                let c = choices.iter()
                    .map(|(b, v)| format!("[bb{}: {}]", b, v))
                    .collect::<Vec<_>>().join(", ");
                write!(f, "  {} = phi({})", dest, c)
            }
            Stmt::Return(None)    => write!(f, "  return"),
            Stmt::Return(Some(v)) => write!(f, "  return {}", v),
            Stmt::Branch { cond, true_bb, false_bb } =>
                write!(f, "  if {} goto bb{} else bb{}", cond, true_bb, false_bb),
            Stmt::Jump(t) =>
                write!(f, "  goto bb{}", t),
            Stmt::Assign { dest, src } =>
                write!(f, "  {} = {}", dest, src),
        }
    }
}

/// A basic block: a sequence of statements with a unique id and the address
/// of its first instruction (useful for mapping back to disassembly).
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub start_addr: u64,
    pub stmts: Vec<Stmt>,
    /// Successor block ids (filled in during CFG construction)
    pub succs: Vec<usize>,
    /// Predecessor block ids
    pub preds: Vec<usize>,
}

impl BasicBlock {
    pub fn new(id: usize, start_addr: u64) -> Self {
        BasicBlock { id, start_addr, stmts: vec![], succs: vec![], preds: vec![] }
    }
}

/// A lifted function: its name (from symbol table if available), entry block id,
/// and the full set of basic blocks.
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub entry: usize,
    pub blocks: Vec<BasicBlock>,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "fn {}:", self.name)?;
        for bb in &self.blocks {
            writeln!(f, "  bb{} (addr={:#x}, preds={:?}, succs={:?}):", bb.id, bb.start_addr, bb.preds, bb.succs)?;
            for s in &bb.stmts {
                writeln!(f, "{}", s)?;
            }
        }
        Ok(())
    }
}