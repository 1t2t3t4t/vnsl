use crate::block_runner::BlockRunner;

#[derive(Debug, Default)]
pub struct RunStack {
    blocks: Vec<BlockRunner>,
}

impl RunStack {
    pub fn top_mut(&mut self) -> Option<&mut BlockRunner> {
        self.blocks.last_mut()
    }

    pub fn push(&mut self, block: BlockRunner) {
        self.blocks.push(block);
    }

    pub fn pop(&mut self) -> Option<BlockRunner> {
        self.blocks.pop()
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}
