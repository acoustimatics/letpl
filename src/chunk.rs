use crate::op::Op;

/// Represents a list of operations runnable on the VM.
pub struct Chunk {
    pub ops: Vec<Op>,
}

impl Chunk {
    /// Creates a chunk that is ready for code generation.
    pub fn new() -> Chunk {
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
            Op::Branch(_) => {
                self.ops[op_index] = Op::Branch(target);
            }
            Op::BranchTrue(_) => {
                self.ops[op_index] = Op::BranchTrue(target);
            }
            _ => (),
        }
    }
}
