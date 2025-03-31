// you'll probably want to expand this...
pub struct FreeBlock {
    start: usize,
    size: usize,
}

impl FreeBlock {
    pub fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }

    // Check if blocks are adjacent
    pub fn is_adjacent(&self, other: &FreeBlock) -> bool {
        self.start + self.size == other.start || other.start + other.size == self.start
    }

    // Merge two adjacent blocks
    pub fn merge(&self, other: &FreeBlock) -> Option<FreeBlock> {
        if self.is_adjacent(other) {
            let new_start = self.start.min(other.start);
            let new_size = self.size + other.size;
            Some(FreeBlock::new(new_start, new_size))
        }
        else {
            None  // Return None if they are not adjacent
        }
    }
    
}
