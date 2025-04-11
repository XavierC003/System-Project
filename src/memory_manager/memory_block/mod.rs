pub mod allocated_block;
pub mod free_block;
pub use free_block::FreeBlock;
// it would be a good idea to make a MemoryBlock trait for AllocatedBlock and
// FreeBlock to inherit from...
//
pub trait MemoryBlock {
    fn get_start(&self) -> usize;
    fn get_size(&self) -> usize;
}

pub fn print_block_info<T: MemoryBlock>(block: &T) {
    println!("Start: {}, Size: {}", block.get_start(), block.get_size());
}
