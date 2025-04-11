use rust_template::file_parser;
use rust_template::memory_manager::memory_block::MemoryBlock;
use std::env;
use std::process;
use rust_template::memory_manager::memory_block::{free_block::FreeBlock, allocated_block::AllocatedBlock};


fn main() {
    // Create a FreeBlock and an AllocatedBlock
    let free_block = FreeBlock::new(1000, 512);
    let allocated_block = AllocatedBlock::new(2000, 256, "block1".to_string(), 256);

    // Print the block information
    print_block_info(&free_block);
    print_block_info(&allocated_block);

    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path.cmmd>");
        process::exit(1);
    }

    let file_path = &args[1];

    // Check if the file has a .cmmd extension
    if !file_path.ends_with(".cmmd") {
        eprintln!("File must have a .cmmd extension");
        process::exit(1);
    }

    // Parse the file using FileParser
    let file_parser = match file_parser::FileParser::new(file_path) {
        Ok(parser) => parser,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    // Print the parsed commands
    for command in file_parser.commands {
        println!("Function: {}, Parameters: {:?}", command.function(), command.parameters());
    }
}

// Function to print block information
fn print_block_info<T: MemoryBlock>(block: &T) {
    println!("Block Start: {}, Block Size: {}", block.get_start(), block.get_size());
}
