// This struct represents an allocated block of memory in the manager.
#[derive(Debug, Clone)]
pub struct AllocatedBlock {
    pub start: usize,       // The starting address (index) of the allocated block in memory
    pub size: usize,        // The total size of the block
    pub id: String,         // Unique identifier for the block
    pub used_size: usize,   // How much of the block is actually being used (data size)
}

// Trait for memory blocks
pub(crate) trait MemoryBlock {
    fn get_start(&self) -> usize;  // Renamed for clarity
    fn get_size(&self) -> usize;     // Renamed for clarity
}

// Implementing the MemoryBlock trait for AllocatedBlock
impl MemoryBlock for AllocatedBlock {
    fn get_start(&self) -> usize {
        self.start
    }

    fn get_size(&self) -> usize {
        self.size
    }
}

impl AllocatedBlock {
    // Creates a new AllocatedBlock
    pub fn new(start: usize, size: usize, id: String, used_size: usize) -> Self {
        Self {
            start,
            size,
            id,
            used_size,
        }
    }

    // This could be a helpful utility function to update used_size
    pub fn update_used_size(&mut self, new_used_size: usize) -> Result<(), String> {
        if new_used_size <= self.size {
            self.used_size = new_used_size;
            Ok(())
        } else {
            Err("new_used_size cannot be larger than the total size of the block".to_string())
        }
    }
}
