use std::collections::{HashMap, HashSet};
use crate::ir::{BasicBlock, Function, Stmt, Operand};
use crate::lifter::{Lifter, CmpState};
use crate::insn::{InsnInfo, OwnedOperand};

fn is_branch(mnemonic: &str) -> bool {
    matches!(mnemonic,
        "jmp" | "je" | "jne" | "jz" | "jnz" | "jl" | "jle" | "jg" | "jge"
        | "jb" | "jbe" | "ja" | "jae" | "js" | "jns" | "jo" | "jno" | "jp" | "jnp"
        | "jcxz" | "jecxz" | "jrcxz" | "loop" | "loope" | "loopne"
    )
}

fn is_unconditional_branch(mnemonic: &str) -> bool {
    matches!(mnemonic, "jmp" | "ret" | "retq" | "ud2")
}

pub fn build_cfg(
    instructions: &[InsnInfo],
    func_name: &str,
    imports: &HashMap<u64, String>,
    rodata: &HashMap<u64, String>,
) -> Function {
    if instructions.is_empty() {
        return Function { name: func_name.to_string(), entry: 0, blocks: vec![] };
    }

    // Pass 1: leaders
    let mut leaders: HashSet<u64> = HashSet::new();
    leaders.insert(instructions[0].address);
    for insn in instructions {
        if is_branch(&insn.mnemonic) || insn.mnemonic == "ret" || insn.mnemonic == "retq" {
            leaders.insert(insn.address + insn.bytes_len as u64);
            if let Some(t) = insn.imm_target { leaders.insert(t); }
        }
    }

    let mut sorted_leaders: Vec<u64> = leaders.into_iter().collect();
    sorted_leaders.sort_unstable();
    let addr_to_block: HashMap<u64, usize> = sorted_leaders.iter()
        .enumerate().map(|(i, &a)| (a, i)).collect();

    let mut blocks: Vec<BasicBlock> = sorted_leaders.iter().enumerate()
        .map(|(id, &addr)| BasicBlock::new(id, addr)).collect();

    // Pass 2: lift
    let mut current_block_id: Option<usize> = None;
    let mut lifter = Lifter::new();

    for insn in instructions {
        if let Some(&block_id) = addr_to_block.get(&insn.address) {
            current_block_id = Some(block_id);
        }
        let block_id = match current_block_id { Some(id) => id, None => continue };

        let stmts = lifter.lift_insn_owned(insn, imports, rodata);
        blocks[block_id].stmts.extend(stmts);

        if is_branch(&insn.mnemonic) || insn.mnemonic == "ret" || insn.mnemonic == "retq" {
            let fall_addr = insn.address + insn.bytes_len as u64;
            let fall_id = addr_to_block.get(&fall_addr).copied();

            if insn.mnemonic == "ret" || insn.mnemonic == "retq" {
                // return emitted by lifter
            } else if is_unconditional_branch(&insn.mnemonic) {
                if let Some(t) = insn.imm_target {
                    if let Some(&tid) = addr_to_block.get(&t) {
                        blocks[block_id].stmts.push(Stmt::Jump(tid));
                        blocks[block_id].succs.push(tid);
                    }
                }
            } else {
                let cond = build_condition(&insn.mnemonic, lifter.last_cmp());
                let tid = insn.imm_target.and_then(|t| addr_to_block.get(&t).copied()).unwrap_or(block_id);
                let fid = fall_id.unwrap_or(block_id + 1);
                blocks[block_id].stmts.push(Stmt::Branch { cond, true_bb: tid, false_bb: fid });
                blocks[block_id].succs.push(tid);
                blocks[block_id].succs.push(fid);
            }
            current_block_id = None;
        }
    }

    // Pass 3: predecessors
    let n = blocks.len();
    for i in 0..n {
        let succs = blocks[i].succs.clone();
        for s in succs { if s < n { blocks[s].preds.push(i); } }
    }

    Function { name: func_name.to_string(), entry: 0, blocks }
}

fn build_condition(mnemonic: &str, cmp: Option<&CmpState>) -> Operand {
    let Some(c) = cmp else { return Operand::Const(1); };
    let s = match mnemonic {
        "je"  | "jz"  => format!("eq({}, {})",     c.left, c.right),
        "jne" | "jnz" => format!("ne({}, {})",     c.left, c.right),
        "jl"           => format!("lt_s({}, {})",   c.left, c.right),
        "jle"          => format!("le_s({}, {})",   c.left, c.right),
        "jg"           => format!("gt_s({}, {})",   c.left, c.right),
        "jge"          => format!("ge_s({}, {})",   c.left, c.right),
        "jb"           => format!("lt_u({}, {})",   c.left, c.right),
        "jbe"          => format!("le_u({}, {})",   c.left, c.right),
        "ja"           => format!("gt_u({}, {})",   c.left, c.right),
        "jae"          => format!("ge_u({}, {})",   c.left, c.right),
        "js"           => format!("sign({}, {})",   c.left, c.right),
        "jns"          => format!("nosign({}, {})", c.left, c.right),
        _              => format!("cond_{}({}, {})", mnemonic, c.left, c.right),
    };
    Operand::Symbol(s, 0)
}