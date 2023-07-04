use std::fmt;

use crate::op::Op;

pub type Address = usize;

/// A VM program.
pub struct Chunk {
    pub ops: Vec<Op>,
}

impl Chunk {
    pub fn new() -> Self {
        let ops = Vec::new();
        Chunk { ops }
    }

    pub fn emit(&mut self, op: Op) -> Address {
        self.ops.push(op);
        self.ops.len() - 1
    }

    pub fn next_address(&self) -> Address {
        self.ops.len()
    }

    pub fn patch(&mut self, patch_at: Address, target: Address) {
        match &self.ops[patch_at] {
            Op::Jump(_) => {
                self.ops[patch_at] = Op::Jump(target);
            }
            Op::JumpTrue(_) => {
                self.ops[patch_at] = Op::JumpTrue(target);
            }
            _ => (),
        }
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "*** chunk {} ops ***", self.ops.len())?;
        for (address, op) in self.ops.iter().enumerate() {
            writeln!(f, "{}\t{:?}", address, op)?;
        }
        Ok(())
    }
}
