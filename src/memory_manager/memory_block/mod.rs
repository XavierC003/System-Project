pub mod allocated_block;
pub mod free_block;

// it would be a good idea to make a MemoryBlock trait for AllocatedBlock and
// FreeBlock to inherit from...
//
pub(crate) trait MemoryBlock {
    fn get_start(&self) -> usize;
    fn get_size(&self) -> usize;
}
