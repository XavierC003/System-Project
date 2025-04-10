use std::io::BufRead;

pub mod command;

#[derive(Debug)]
pub struct FileParser {
    commands: Vec<command::Command>,
}

impl FileParser {
    pub fn new(file_path: &str) -> Result<FileParser, std::io::Error> {
        // read from the file and get the commands
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        
        let mut commands = Vec::new();

        // Read each line from the file
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if !parts.is_empty() {
                let function = parts[0].to_string();
                let parameters = parts[1..].iter().map(|&s| s.to_string()).collect();
                commands.push(command::Command::new(function, parameters));
            }
        }
        Ok(FileParser { commands })
    }
}
