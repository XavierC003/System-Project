use crate::memory_manager::MemoryManager;
use crate::memory_manager::memory_block::allocated_block::AllocatedBlock;

impl MemoryManager {
        /// Reads and prints information about a specific allocated memory block.
    ///
    /// This function retrieves a block from the allocated handles by its unique ID.
    /// It prints details about the block, including the start and end addresses, size, used size, and its status.
    /// The function also prints the block's data in hexadecimal format up to the `used_size`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the allocated block. This is a `usize` representing the index of the block
    ///   in the `allocated_handles` vector.
    ///
    /// # Returns
    ///
    /// * `Option<&AllocatedBlock>` - If the block with the given ID exists, it returns a reference to the `AllocatedBlock`.
    ///   If the block doesn't exist, it returns `None`.
    pub fn read(&self, id: usize) -> Option<&AllocatedBlock> {
        if let Some(block) = self.allocated_handles.get(id) {
            // Calculate the addresses
            let start_addr = block.start;
            let end_addr = block.start + block.size - 1;

            // Print the block information in the required format
            println!(
                "Allocated Block: ID = {}, Start = {:04x}, End = {:04x}, Size = {}, Used = {}, Status = ALLOCATED",
                id,
                start_addr,
                end_addr,
                block.size,
                block.used_size
            );

            // Print the data as hexadecimal bytes up to used size
            let data = &self.data[start_addr..=end_addr];
            print!("Data: ");
            for byte in data.iter().take(block.used_size) {
                print!("{:02x} ", byte);
            }
            println!();

            Some(block)
        } else {
            // Print when the block isn't found
            println!("Nothing at {}", id);
            None
        }
    }
}
