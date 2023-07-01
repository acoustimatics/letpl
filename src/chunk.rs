use std::fmt;

use crate::op::Op;

/// Represents a list of operations runnable on the VM.
pub struct Chunk {
    pub ops: Vec<Op>,
}

impl Chunk {
    /// Creates a chunk that is ready for code generation.
    pub fn new() -> Self {
        let ops = Vec::new();
        Chunk { ops }
    }

    /// Includes an Op at the end of the chunk. Returns the Op's index in the
    /// chunk.
    pub fn emit(&mut self, op: Op) -> usize {
        self.ops.push(op);
        self.ops.len() - 1
    }

    /// Returns the index of the next emitted Op.
    pub fn next_index(&self) -> usize {
        self.ops.len()
    }

    /// Patches a branch at and Op index with a given index into the chunk.
    pub fn patch(&mut self, op_index: usize, target: usize) {
        match &self.ops[op_index] {
            Op::Jump(_) => {
                self.ops[op_index] = Op::Jump(target);
            }
            Op::JumpTrue(_) => {
                self.ops[op_index] = Op::JumpTrue(target);
            }
            _ => (),
        }
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "*** chunk {} ops ***", self.ops.len())?;
        for (i, op) in self.ops.iter().enumerate() {
            writeln!(f, "{}\t{:?}", i, op)?;
        }
        Ok(())
    }
}
