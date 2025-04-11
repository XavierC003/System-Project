// This struct represents a free block of memory (unused memory space).
use super::MemoryBlock;

#[derive(Debug, Clone)]
pub struct FreeBlock {
    start: usize, // The starting address (index) of the free block in memory
    size: usize,  // The total size of the free block
}

impl FreeBlock {
    // Creates a new FreeBlock
    pub fn new(start: usize, size: usize) -> Self {
        Self { start, size }
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
