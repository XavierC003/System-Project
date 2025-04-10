// This struct represents a free block of memory (unused memory space).
use super::MemoryBlock;

#[derive(Debug, Clone)]
pub struct FreeBlock {
    pub start: usize, // The starting address (index) of the free block in memory
    pub size: usize,  // The total size of the free block
}

impl FreeBlock {
    // Creates a new FreeBlock
    pub fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }

    // Merges two free blocks if they are adjacent
    // Returns Some(FreeBlock) if the blocks were merged, None otherwise
    pub fn merge(&self, other: &FreeBlock) -> Option<FreeBlock> {
        if self.start + self.size == other.start {
            Some(FreeBlock::new(self.start, self.size + other.size))
        } else {
            None
        }
    }
}
    // Implementing the MemoryBlock trait for FreeBlock
    impl MemoryBlock for FreeBlock {
        fn get_start(&self) -> usize {
            self.start
        }
    
        fn get_size(&self) -> usize {
            self.size
        }
    }
