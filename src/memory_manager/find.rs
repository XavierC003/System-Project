use crate::memory_block::allocated_block::AllocatedBlock;

pub fn find_block(blocks: &[AllocatedBlock], id: &str, memory: &[u8]) {
    if let Some(block) = blocks.iter().find(|b| b.id == id) {
        println!("Start Address: {:04X}", block.start);
        println!("Status: ALLOCATED");
        println!("ID: {}", block.id);
        println!("Size: {}", block.size);
        println!("Used Size: {}", block.used_size);

        let data = &memory[block.start..block.start + block.used_size];
        print!("Data: ");
        for byte in data {
            print!("{:02X} ", byte);
        }
        println!();
    } else {
        println!("Nothing at {}", id);
    }
}

