use capstone::arch::x86::X86Reg;
use std::collections::HashMap;
use crate::ir::{Var, Operand, BinOp, Stmt};
use crate::insn::{InsnInfo, OwnedOperand};

pub struct Lifter {
    next_var: usize,
    reg_map: HashMap<u32, Var>,
    pub last_cmp: Option<CmpState>,
    /// Tracks registers that were loaded from a known GOT slot.
    /// When "mov rax, [rip+got_slot]" is seen and got_slot resolves to a symbol,
    /// we record rax -> symbol_name so "call rax" can resolve it.
    got_reg_map: HashMap<u32, String>,
}

#[derive(Debug, Clone)]
pub struct CmpState {
    pub left: Operand,
    pub right: Operand,
}

impl Lifter {
    pub fn new() -> Self {
        Lifter { next_var: 0, reg_map: HashMap::new(), last_cmp: None, got_reg_map: HashMap::new() }
    }

    fn fresh(&mut self) -> Var {
        let v = Var(self.next_var);
        self.next_var += 1;
        v
    }

    fn read_reg(&mut self, reg: u32) -> Operand {
        if reg == 0 { return Operand::Const(0); }
        let v = self.reg_map.entry(reg).or_insert_with(|| {
            let id = self.next_var;
            self.next_var += 1;
            Var(id)
        }).clone();
        Operand::Var(v)
    }

    fn write_reg(&mut self, reg: u32) -> Var {
        let v = Var(self.next_var);
        self.next_var += 1;
        self.reg_map.insert(reg, v.clone());
        v
    }

    pub fn last_cmp(&self) -> Option<&CmpState> {
        self.last_cmp.as_ref()
    }

    /// Lift an owned InsnInfo into IR statements.
    pub fn lift_insn_owned(
        &mut self,
        insn: &InsnInfo,
        imports: &HashMap<u64, String>,
        rodata: &HashMap<u64, String>,
    ) -> Vec<Stmt> {
        let ops = &insn.operands;
        match insn.mnemonic.as_str() {
            // ---- Data movement ----
            "mov" | "movabs" | "movzx" | "movsx" | "movsxd" => {
                if ops.len() < 2 { return vec![]; }
                // Check if this is a GOT load: mov reg, [rip+N]
                // If so, record the dest register -> import name for call resolution.
                if let OwnedOperand::Mem { base, disp, .. } = &ops[1] {
                    // RIP-relative: capstone sets base = X86_REG_RIP (41), not 0.
                    if *base == 41 {
                        let got_slot = (insn.address + insn.bytes_len as u64).wrapping_add(*disp as u64);
                        if let Some(sym) = imports.get(&got_slot) {
                            if let OwnedOperand::Reg(dest_reg) = &ops[0] {
                                self.got_reg_map.insert(*dest_reg, sym.clone());
                            }
                        }
                    }
                }
                let src = self.read_op(&ops[1]);
                self.write_op_assign(&ops[0], src)
            }

            "lea" => {
                if ops.len() < 2 { return vec![]; }
                // If RIP-relative and address is in .rodata, emit string literal.
                if let OwnedOperand::Mem { base, disp, .. } = &ops[1] {
                    if *base == 41 { // X86_REG_RIP
                        let str_addr = (insn.address + insn.bytes_len as u64)
                            .wrapping_add(*disp as u64);
                        if let Some(s) = rodata.get(&str_addr) {
                            let sym = Operand::Symbol(s.clone(), 0);
                            return self.write_op_assign(&ops[0], sym);
                        }
                    }
                }
                let addr = self.compute_addr(&ops[1]);
                self.write_op_assign(&ops[0], addr)
            }

            "push" => {
                if ops.is_empty() { return vec![]; }
                let src = self.read_op(&ops[0]);
                let rsp = self.read_reg(X86Reg::X86_REG_RSP as u32);
                vec![Stmt::Store { base: rsp, offset: -8, src, size: 8 }]
            }

            "pop" => {
                if ops.is_empty() { return vec![]; }
                let rsp = self.read_reg(X86Reg::X86_REG_RSP as u32);
                let dest = self.write_op_var(&ops[0]);
                vec![Stmt::Load { dest, base: rsp, offset: 0, size: 8 }]
            }

            // ---- Arithmetic ----
            "add"  => self.binop(ops, BinOp::Add),
            "sub"  => self.binop(ops, BinOp::Sub),
            "imul" => self.binop(ops, BinOp::Mul),
            "and"  => self.binop(ops, BinOp::And),
            "or"   => self.binop(ops, BinOp::Or),
            "shl" | "sal" => self.binop(ops, BinOp::Shl),
            "shr"  => self.binop(ops, BinOp::Shr),
            "sar"  => self.binop(ops, BinOp::Sar),

            "xor" => {
                // xor reg, reg is the zero idiom
                if ops.len() >= 2 {
                    if let (OwnedOperand::Reg(r1), OwnedOperand::Reg(r2)) = (&ops[0], &ops[1]) {
                        if r1 == r2 {
                            let dest = self.write_reg(*r1);
                            return vec![Stmt::Assign { dest, src: Operand::Const(0) }];
                        }
                    }
                }
                self.binop(ops, BinOp::Xor)
            }

            "inc" => {
                if ops.is_empty() { return vec![]; }
                let left = self.read_op(&ops[0]);
                let dest = self.write_op_var(&ops[0]);
                vec![Stmt::BinOp { dest, op: BinOp::Add, left, right: Operand::Const(1) }]
            }
            "dec" => {
                if ops.is_empty() { return vec![]; }
                let left = self.read_op(&ops[0]);
                let dest = self.write_op_var(&ops[0]);
                vec![Stmt::BinOp { dest, op: BinOp::Sub, left, right: Operand::Const(1) }]
            }
            "neg" => {
                if ops.is_empty() { return vec![]; }
                let right = self.read_op(&ops[0]);
                let dest = self.write_op_var(&ops[0]);
                vec![Stmt::BinOp { dest, op: BinOp::Sub, left: Operand::Const(0), right }]
            }

            // ---- Comparisons ----
            "cmp" | "test" => {
                if ops.len() < 2 { return vec![]; }
                let left = self.read_op(&ops[0]);
                let right = self.read_op(&ops[1]);
                self.last_cmp = Some(CmpState { left, right });
                vec![]
            }

            // ---- Calls ----
            "call" => {
                let target = if let Some(addr) = insn.imm_target {
                    // Direct call: target is an immediate address.
                    imports.get(&addr).cloned().unwrap_or_else(|| format!("fn_{:#x}", addr))
                } else if let Some(OwnedOperand::Mem { base, disp, .. }) = ops.first() {
                    // call [rip + N] or call [reg + N]: GOT-indirect call (-fno-plt style).
                    // Compute the GOT slot address: for RIP-relative, base is RIP (0 in capstone
                    // memory operands means RIP for RIP-relative), disp is the offset.
                    // The actual GOT slot VA = insn.address + insn.bytes_len + disp.
                    let got_slot = if *base == 41 { // X86_REG_RIP
                        // RIP-relative: capstone sets base=X86_REG_RIP(41)
                        (insn.address + insn.bytes_len as u64).wrapping_add(*disp as u64)
                    } else {
                        *disp as u64 // absolute or register-relative: use disp as hint
                    };
                    imports.get(&got_slot).cloned().unwrap_or_else(|| format!("got_{:#x}", got_slot))
                } else if let Some(OwnedOperand::Reg(r)) = ops.first() {
                    // Check if this register was loaded from a known GOT slot.
                    if let Some(sym) = self.got_reg_map.get(r).cloned() {
                        let args = self.abi_args();
                        let dest = self.write_reg(X86Reg::X86_REG_RAX as u32);
                        return vec![Stmt::Call { dest: Some(dest), target: sym, args }];
                    }
                    let ptr = self.read_reg(*r);
                    let args = self.abi_args();
                    let dest = self.write_reg(X86Reg::X86_REG_RAX as u32);
                    return vec![Stmt::IndirectCall { dest: Some(dest), ptr, args }];
                } else {
                    format!("fn_{:#x}", insn.address)
                };

                let args = self.abi_args();
                let dest = self.write_reg(X86Reg::X86_REG_RAX as u32);
                vec![Stmt::Call { dest: Some(dest), target, args }]
            }

            "ret" | "retq" => {
                let ret_val = self.read_reg(X86Reg::X86_REG_RAX as u32);
                vec![Stmt::Return(Some(ret_val))]
            }

            "nop" | "endbr64" | "endbr32" => vec![],

            _ => vec![],
        }
    }

    // ---- Helpers ----

    fn binop(&mut self, ops: &[OwnedOperand], op: BinOp) -> Vec<Stmt> {
        if ops.len() < 2 { return vec![]; }
        let left = self.read_op(&ops[0]);
        let right = self.read_op(&ops[1]);
        let dest = self.write_op_var(&ops[0]);
        vec![Stmt::BinOp { dest, op, left, right }]
    }

    fn read_op(&mut self, op: &OwnedOperand) -> Operand {
        match op {
            OwnedOperand::Reg(r) => self.read_reg(*r),
            OwnedOperand::Imm(i) => Operand::Const(*i),
            OwnedOperand::Mem { base, disp, .. } => {
                // Emit an implicit load; return a fresh var representing the value.
                // Full treatment requires threading Vec<Stmt> through here.
                let _base_op = self.read_reg(*base);
                let dest = self.fresh();
                Operand::Var(dest)
            }
        }
    }

    fn write_op_var(&mut self, op: &OwnedOperand) -> Var {
        match op {
            OwnedOperand::Reg(r) => self.write_reg(*r),
            _ => self.fresh(),
        }
    }

    fn write_op_assign(&mut self, op: &OwnedOperand, src: Operand) -> Vec<Stmt> {
        match op {
            OwnedOperand::Reg(r) => {
                let dest = self.write_reg(*r);
                vec![Stmt::Assign { dest, src }]
            }
            OwnedOperand::Mem { base, disp, .. } => {
                let base_op = self.read_reg(*base);
                vec![Stmt::Store { base: base_op, offset: *disp, src, size: 8 }]
            }
            _ => vec![],
        }
    }

    fn compute_addr(&mut self, op: &OwnedOperand) -> Operand {
        match op {
            OwnedOperand::Mem { base, disp, .. } => {
                let base_op = self.read_reg(*base);
                if *disp != 0 {
                    // Ideally emit a BinOp Add here; for now return base.
                    // Consumers use the disp field on Load/Store directly.
                    base_op
                } else {
                    base_op
                }
            }
            _ => self.read_op(op),
        }
    }

    fn abi_args(&mut self) -> Vec<Operand> {
        [
            X86Reg::X86_REG_RDI,
            X86Reg::X86_REG_RSI,
            X86Reg::X86_REG_RDX,
            X86Reg::X86_REG_RCX,
            X86Reg::X86_REG_R8,
            X86Reg::X86_REG_R9,
        ].iter().map(|&r| self.read_reg(r as u32)).collect()
    }
}