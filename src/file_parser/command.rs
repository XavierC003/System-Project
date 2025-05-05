use crate::memory_manager::MemoryManager;

#[derive(Debug)]
pub struct Command {
    function: String,
    parameters: Vec<String>,
}

impl Command {
    pub fn new(function: String, parameters: Vec<String>) -> Command {
        Command {
            function,
            parameters,
        }
    }

    pub fn function(&self) -> &str {
        &self.function
    }

    pub fn parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn execute(&self, memory_manager: &mut MemoryManager) {
        match self.function.as_str() {
            "INSERT" => {
                if self.parameters.len() >= 2 {
                    let size = self.parameters[0].parse::<usize>().unwrap_or(0);
                    let data = self.parameters[1..].join(" ");
                    memory_manager.insert(size, data.as_bytes());
                } else {
                    println!("INSERT missing parameters");
                }
            }

            "READ" => {
                if let Some(id_str) = self.parameters.get(0) {
                    if let Ok(id) = id_str.parse::<usize>() {
                        memory_manager.read(id);
                    } else {
                        println!("Invalid ID in READ");
                    }
                }
            }

            "UPDATE" => {
                if self.parameters.len() >= 2 {
                    if let Ok(id) = self.parameters[0].parse::<usize>() {
                        let new_data = self.parameters[1..].join(" ");
                        MemoryManager::update(memory_manager, id, new_data.as_bytes())
                    } else {
                        println!("Invalid ID in UPDATE");
                    }
                } else {
                    println!("UPDATE missing parameters");
                }
            }

            "DELETE" => {
                if let Some(id_str) = self.parameters.get(0) {
                    if let Ok(id) = id_str.parse::<usize>() {
                        memory_manager.delete(id);
                    } else {
                        println!("Invalid ID in DELETE");
                    }
                }
            }

            "DUMP" => {
                memory_manager.dump();
            }

            _ => println!("Unknown command: {}", self.function),
        }
    }
}
