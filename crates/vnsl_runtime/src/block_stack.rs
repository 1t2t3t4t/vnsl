use vnsl_core::model::VnslBlock;

#[derive(Debug, Default)]
pub struct BlockStack {
    blocks: Vec<VnslBlock>,
}

impl BlockStack {
    pub fn top(&self) -> Option<&VnslBlock> {
        self.blocks.last()
    }

    pub fn push(&mut self, block: VnslBlock) {
        self.blocks.push(block);
    }

    pub fn pop(&mut self) -> Option<VnslBlock> {
        self.blocks.pop()
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}
