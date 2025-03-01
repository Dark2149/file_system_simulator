use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum MyErrors {
    BadPath,
    BadDirectory,
}

impl fmt::Display for MyErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyErrors::BadPath => write!(f, "Bad path, check your spelling"),
            MyErrors::BadDirectory => write!(f, "Is not a directory"),
        }
    }
}

impl std::error::Error for MyErrors {}

#[derive(Debug)]
enum Entry {
    File(String),
    Directory(HashMap<String, Entry>),
}

#[derive(Debug)]
struct FileSystem {
    root: Entry,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: Entry::Directory(HashMap::new()),
        }
    }

    fn create_file(&mut self, path: &str, content: &str) -> Result<(), MyErrors> {
        let parts: Vec<&str> = path.split('/').filter(|&part| !part.is_empty()).collect();

        if parts.is_empty() {
            return Err(MyErrors::BadPath);
        }

        let file_name = parts.last().unwrap().to_string();
        let dir_path = parts[..parts.len() - 1].join("/");

        let dir = self.get_directory_mut(&dir_path)?;
        dir.insert(file_name, Entry::File(content.to_string()));
        Ok(())
    }

    fn get_directory_mut(&mut self, path: &str) -> Result<&mut HashMap<String, Entry>, MyErrors> {
        let parts: Vec<&str> = path.split('/').filter(|&part| !part.is_empty()).collect();
        let mut current = &mut self.root;

        for part in parts {
            current = match current {
                Entry::Directory(entries) => {
                    if !entries.contains_key(part) {
                        entries.insert(part.to_string(), Entry::Directory(HashMap::new()));
                    }
                    entries.get_mut(part).unwrap()
                }
                _ => return Err(MyErrors::BadDirectory),
            };
        }

        if let Entry::Directory(dir) = current {
            Ok(dir)
        } else {
            Err(MyErrors::BadDirectory)
        }
    }
}

fn main() {
    let mut fs = FileSystem::new();

    // Создаем файлы
    match fs.create_file("/test/file1.txt", "Содержимое file1.txt") {
        Ok(_) => println!("Файл создан"),
        Err(e) => eprintln!("Ошибка создания: {}", e),
    }

    match fs.create_file("/test/file2.txt", "Содержимое file2.txt") {
        Ok(_) => println!("Файл создан"),
        Err(e) => eprintln!("Ошибка создания: {}", e),
    }

    match fs.create_file("/test/subdir/file3.txt", "Содержимое file3.txt") {
        Ok(_) => println!("Файл создан"),
        Err(e) => eprintln!("Ошибка создания: {}", e),
    }

    match fs.create_file("/another_dir/file5.txt", "Содержимое file5.txt") {
        Ok(_) => println!("Файл создан"),
        Err(e) => eprintln!("Ошибка создания: {}", e),
    }

    // Выводим содержимое файловой системы
    println!("{:?}", fs);
}