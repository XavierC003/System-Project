// you'll probably want to expand this...
pub struct AllocatedBlock {
    start: usize,   // starting address of the block
    end: usize,     // ending address of the block
    size: usize,    // total size of the block
    id: String,    // unique identifier for the block
    used_size: usize,   // size of the block that is currently used
}

impl AllocatedBlock {
    pub fn new(start:usize, size: usize, id: String, used_size: usize) -> Self {
        Self { start, size, id, used_size}
    }
}
