use capstone::prelude::*;
use capstone::arch::x86::X86OperandType as OpTy;

#[derive(Debug, Clone)]
pub struct InsnInfo {
    pub address: u64,
    pub mnemonic: String,
    pub bytes_len: usize,
    pub imm_target: Option<u64>,
    pub operands: Vec<OwnedOperand>,
}

#[derive(Debug, Clone)]
pub enum OwnedOperand {
    Reg(u32),
    Imm(i64),
    Mem { base: u32, index: u32, scale: i32, disp: i64 },
}

impl InsnInfo {
    pub fn extract(insn: &capstone::Insn, detail: &capstone::InsnDetail) -> Self {
        let mnemonic = insn.mnemonic().unwrap_or("").to_string();
        let arch = detail.arch_detail();
        let operands: Vec<OwnedOperand> = if let Some(x86) = arch.x86() {
            x86.operands().map(|op| match op.op_type {
                OpTy::Reg(r) => OwnedOperand::Reg(r.0 as u32),
                OpTy::Imm(i) => OwnedOperand::Imm(i),
                OpTy::Mem(m) => OwnedOperand::Mem {
                    base:  m.base().0 as u32,
                    index: m.index().0 as u32,
                    scale: m.scale(),
                    disp:  m.disp(),
                },
                _ => OwnedOperand::Imm(0),
            }).collect()
        } else {
            vec![]
        };

        let imm_target = operands.iter().find_map(|op| {
            if let OwnedOperand::Imm(i) = op { Some(*i as u64) } else { None }
        });

        InsnInfo {
            address: insn.address(),
            mnemonic,
            bytes_len: insn.bytes().len(),
            imm_target,
            operands,
        }
    }
}