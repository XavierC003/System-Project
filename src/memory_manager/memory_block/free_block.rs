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

    pub fn can_allocate(&self, requested_size: usize) -> bool {
        self.size >= requested_size
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn set_size(&mut self, new_size: usize) {
        self.size = new_size;
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn set_start(&mut self, new_start: usize) {
        self.start = new_start;
    }
    pub fn is_buddy_of(&self, other: &FreeBlock) -> bool {
        self.size == other.size && (self.start ^ other.start) == self.size
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
