use core::error;
use std::{ascii::AsciiExt, collections::HashMap};

enum Errors {
    InvalidPathe(String)
}
#[derive(Debug)]
enum Entry {
    File(String),
    Directory(HashMap<String, Entry>)
}

struct FileSystem {
    root: Entry,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: Entry::Directory(HashMap::new()),
        }
    }

    fn create_file(&mut self, path: &str, content: &str) -> Result<(), String> {
        let parts: Vec<&str> = path.split('/').filter(|&part| !part.is_empty()).collect();

        if parts.is_empty() {
            return Err("Invalid path".to_string())
        };

        let file_name = parts.last().unwrap().to_string();
        let dir_path = parts[..parts.len() - 1].join("/");

        let dir = self.get_directory_mut(&dir_path)?;
        dir.insert(file_name, Entry::File(content.to_string()));
        Ok(())
    }
}