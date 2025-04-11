use rust_template::file_parser;
use rust_template::memory_manager::MemoryManager;
use rust_template::memory_manager::memory_block::MemoryBlock; // Added for MemoryBlock trait

use std::env;
use std::process;




fn main() {
    let mut manager = MemoryManager::new(1024); // Create memory manager

    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path.cmmd>");
        process::exit(1);
    }
    let file_path = &args[1];

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

    // Process commands from file
    for command in file_parser.commands {
        match command.function() {
            "insert" => {
                if let Some(size_str) = command.parameters().get(0) {
                    match size_str.parse::<usize>() {
                        Ok(size) => {
                            let data: Vec<u8> = command.parameters()[1..]
                                .iter()
                                .filter_map(|s| s.parse::<u8>().ok())
                                .collect();
                            if let Some(id) = manager.insert(size, &data) {
                                println!("Inserted block with ID: {}", id);
                                // Assuming you have a function to get a block by ID
                                if let Some(block) = manager.get_block(&id) {
                                    print_block_info(block);
                                }
                            } else {
                                println!("Failed to insert block.");
                            }
                        }
                        Err(_) => {
                            println!("Invalid size parameter for insert command.");
                        }
                    }
                } else {
                    println!("Missing size parameter for insert command.");
                }
            }
            "delete" => {
                if let Some(id) = command.parameters().get(0) {
                    match id.parse::<usize>() {
                        Ok(parsed_id) => {
                            if manager.delete(&id) {
                                println!("Deleted block with ID: {}", parsed_id);
                            } else {
                                println!("Block ID not found: {}", parsed_id);
                            }
                        }
                        Err(_) => {
                            println!("Invalid block ID for delete command.");
                        }
                    }
                } else {
                    println!("Missing block ID for delete command.");
                }
            }
            "read" => {
                if let (Some(start_str), Some(len_str)) = (command.parameters().get(0), command.parameters().get(1)) {
                    let start = start_str.parse::<usize>().unwrap_or(0);
                    let len = len_str.parse::<usize>().unwrap_or(1);
                    match manager.read_range(start, len) {
                        Some(bytes) => println!("Bytes from {}: {:?}", start, bytes),
                        None => println!("Invalid index: {} to {}", start, start + len),
                    }
                } else {
                    println!("Missing parameters for read command.");
                }
            }
            _ => {
                println!("Unknown command: {}", command.function());
            }
        }
    }
}

// Function to print block information
fn print_block_info(block: &impl MemoryBlock) {
    println!("Block Start: {}, Block Size: {}", block.get_start(), block.get_size());
}
