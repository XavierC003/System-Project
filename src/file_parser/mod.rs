pub mod command;

#[derive(Debug)]
pub struct FileParser {
    commands: Vec<command::Command>,
}

impl FileParser {
    pub fn new(file_path: &str) -> Result<FileParser, std::io::Error> {
        // read from the file and get the commands
        todo!("not implemented")
    }
}
