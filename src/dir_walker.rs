use mime_guess;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Weak, Mutex};

#[derive(Clone)]
pub struct File {
    name: String,
    size: u64,
    mime: String
}

impl File {
    fn new(name: &str, size: u64, mime: &str) -> File {
        File {
            name: name.to_string(),
            size: size,
            mime: mime.to_string()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_mime(&self) -> &str {
        &self.mime
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
    directories: Vec<Arc<Mutex<Directory>>>,
    files: Vec<File>,
    parent: Weak<Mutex<Directory>>
}

impl Directory {
    fn new(name: &str, parent: Weak<Mutex<Directory>>) -> Directory {
        Directory {
            name: name.to_string(),
            size: 0,
            directories: vec![],
            files: vec![],
            parent: parent
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_subdirectories(&self) -> &Vec<Arc<Mutex<Directory>>> {
        &self.directories
    }

    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn get_parent(&self) -> Weak<Mutex<Directory>> {
        self.parent.clone()
    }

    fn set_subdirectories(&mut self, subdirs: Vec<Arc<Mutex<Directory>>>) {
        self.directories = subdirs;
    }

    fn set_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    fn set_size(&mut self, size: u64) {
        self.size = size;
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sub_strings = self.directories.iter().map(|ent| ent.lock().unwrap().to_string()).collect::<Vec<String>>().join("\n");
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

fn read_dir_impl(path: &PathBuf, parent: Weak<Mutex<Directory>>) -> std::io::Result<Arc<Mutex<Directory>>> {
    let root_name = match path_get_file_name(&path) {
        Some(n) => n,
        None => "".to_string()
    };

    let directory = Arc::new(Mutex::new(Directory::new(&root_name, parent)));
    let mut subdirectories: Vec<Arc<Mutex<Directory>>> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut size: u64 = 0;
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let metadata = entry.metadata()?;
            size += metadata.len();

            if let Ok(name) = entry.file_name().into_string() {
                if metadata.is_file() {
                    let mime = mime_guess::from_path(entry.path()).first_or_text_plain()
                                                                  .to_string();
                    files.push(File::new(&name, metadata.len(), &mime)); 
                }
                else if metadata.is_dir() {
                    if let Ok(dir) = read_dir_impl(&entry.path(), Arc::downgrade(&directory)) {
                        size += dir.lock().unwrap().size;
                        subdirectories.push(dir);
                    }
                }
            }
        }
    }

    if let Ok(mut unwrapped_dir) = directory.lock() {
        unwrapped_dir.set_subdirectories(subdirectories);
        unwrapped_dir.set_files(files);
        unwrapped_dir.set_size(size);
    }

    Ok(directory)
}

pub fn read_dir(path: &PathBuf) -> std::io::Result<Arc<Mutex<Directory>>> {
    read_dir_impl(path, Weak::new())
}
