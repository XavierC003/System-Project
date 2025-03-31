pub mod command;

#[derive(Debug)]
struct FileParser {
    commands: Vec<command::Command>,
}

impl FileParser {
    pub fn new(file_path: &str) -> FileParser {
        // read from the file and get the commands
        todo!("not implemented")
    }
}
