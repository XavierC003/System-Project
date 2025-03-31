mod memory_block;

use memory_block::allocated_block::AllocatedBlock;
use memory_block::free_block::FreeBlock;
use crate::memory_manager::memory_block::MemoryBlock;

pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
    pub next_id: usize, // id counter
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            memort: vec![0; 65535], // Initialize with 64KB of memory
            free_handles: vec![FreeBlock::new(0, 65535)], // Start with one large free block
            allocated_handles: vec![FreeBlock::new(0, 65535)], // No allocated blocks initially
            next_id: 0, // Initialize the ID counter
        }
    }

    // Insert
    pub fn insert(&mut self, size: usize, data: &[u8]) -> Option<String> {
        let required_size = size.next_power_of_two(); // Round up to the next power of two

        if let Some(index) = self.free_blocks.iter().position(|b| b.size >= required_size) {
            let free_block = self.free_blocks.remove(index); // Remove the free block
            let id = slef.next_id.to_string(); // Create a unique ID for the allocated block
            self.next_id += 1; // Increment the ID counter

            self.allocated_blocks.push(AllocatedBlock::new(
                free_block.start,
                required_size,
                id.clone(),
                size, // Store the actual used size
            ));
            // Copy the data into the allocated block
            self.memory[free_block.start..free_block.start + size]
                .copy_from_slice(data);
        

            if free_block.size > required_size {
            // If there is leftover space, create a new free block
                let leftover_size = free_block.size - required_size;
                self.free_blocks.push(FreeBlock::new(
                free_block.start + required_size,
                leftover_size,
            ));
        }

        return Some(id); // Return the unique ID of the allocated block
         }
        None // Not enough memory
        }
        
    
    // Free blocks are merged after allocation
    pub fn merge_free_blocks(&mut self) {
        self.free_blocks.sort_by_key(|b| b.start); // Sort free blocks by start address
    
        let mut merged = Vec::new();
        let mut prev: Option<FreeBlock> = None;

        for block in &self.free_blocks.drain[..] {
            if let Some(p) = prev {
                if let Some(merged_block) = p.merge(block) {
                    prev = Some(merged_block); // Merge with the previous block
                } else {
                    merged.push(p); // Add the previous block to the merged list
                    prev = Some(block.clone()); // Start a new block
                }
            }
        }
        if let Some(p) = prev {
            merged.push(p); // Add the last block to the merged list
        }
        self.free_blocks = merged; // Update the free blocks list
    }
    
    // Delete
    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(index) = self.allocated_blocks.iter().position(|b| b.id == id) {
            let block = self.allocated_blocks.remove(index); // Remove the allocated block

            self.free_blocks.push(FreeBlock::new(
                block.start,
                block.size,
            )); // Add the freed block to the free list

            // Merge adjacent free blocks
            self.merge_free_blocks();
            return true;
        }
        false
    }
    // Dump
    pub fn dump(&self) {
        println!("Allocated Blocks:");
        for block in &self.allocated_blocks {
            println!("ID: {}, Start: {}, Size: {}, Used: {}", block.id, block.start, block.size, block.used_size);
        }

        println!("Free Blocks:");
        for block in &self.free_blocks {
            println!("Start: {}, Size: {}", block.start, block.size);
        }
    }
}
