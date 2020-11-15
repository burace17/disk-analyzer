use std::fmt;
use std::fs;
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Clone)]
pub struct File {
    name: String,
    size: u64
}

impl File {
    fn new(name: &str, size: u64) -> File {
        File {
            name: name.to_string(),
            size: size
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.size)
    }
}

#[derive(Clone)]
pub struct Directory {
    name: String,
    size: u64,
    directories: Vec<Arc<Directory>>,
    files: Vec<File>
}

impl Directory {
    fn new(name: &str, size: u64, directories: Vec<Arc<Directory>>, files: Vec<File>) -> Directory {
        Directory {
            name: name.to_string(),
            size: size,
            directories: directories,
            files: files
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_subdirectories(&self) -> &Vec<Arc<Directory>> {
        &self.directories
    }

    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sub_strings = self.directories.iter().map(|ent| ent.to_string()).collect::<Vec<String>>().join("\n");
        let file_strings = self.files.iter().map(|ent| ent.to_string()).collect::<Vec<String>>().join("\n");
        write!(f, "----- {} {} ------\n{}\n{}", self.name, self.size, sub_strings, file_strings)
    }
}

fn path_get_file_name(path: &PathBuf) -> Option<String> {
    if let Some(osstr) = path.file_name() {
        match osstr.to_os_string().into_string() {
            Ok(file_name) => Some(file_name),
            Err(_) => None
        }
    }
    else {
        None
    }
}

pub fn read_dir(path: &PathBuf) -> std::io::Result<Directory> {
    let mut subdirectories: Vec<Arc<Directory>> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut size: u64 = 0;
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let metadata = entry.metadata()?;
            size += metadata.len();

            if let Ok(name) = entry.file_name().into_string() {
                if metadata.is_file() {
                    files.push(File::new(&name, metadata.len())); 
                }
                else if metadata.is_dir() {
                    if let Ok(dir) = read_dir(&entry.path()) {
                        size += dir.size;
                        subdirectories.push(Arc::new(dir));
                    }
                }
            }
        }
    }

    let root_name = match path_get_file_name(&path) {
        Some(n) => n,
        None => "".to_string()
    };
    Ok(Directory::new(&root_name, size, subdirectories, files))
}
