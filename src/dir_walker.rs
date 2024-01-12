// /* This Source Code Form is subject to the terms of the Mozilla Public
//  * License, v. 2.0. If a copy of the MPL was not distributed with this
//  * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use mime_guess;
// use std::fmt;
// use std::fs;
// use std::path::PathBuf;
// use std::sync::{Arc, Weak, Mutex};
// use std::sync::mpsc::Receiver;
// use thiserror::Error;

// #[derive(Error, Debug, Clone)]
// pub enum ReadError {
//     #[error("I/O error")]
//     IOError(std::io::ErrorKind),
//     #[error("Operation cancelled")]
//     OperationCancelled,
// }

// #[derive(Clone)]
// pub struct File {
//     name: String,
//     size: u64,
//     mime: String
// }

// impl File {
//     fn new(name: &str, size: u64, mime: &str) -> File {
//         File {
//             name: name.to_string(),
//             size: size,
//             mime: mime.to_string()
//         }
//     }

//     pub fn get_name(&self) -> &str {
//         &self.name
//     }

//     pub fn get_size(&self) -> u64 {
//         self.size
//     }

//     pub fn get_mime(&self) -> &str {
//         &self.mime
//     }
// }

// impl fmt::Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} {}", self.name, self.size)
//     }
// }

// #[derive(Clone)]
// pub struct Directory {
//     name: String,
//     size: u64,
//     directories: Vec<Arc<Mutex<Directory>>>,
//     files: Vec<File>,
//     parent: Weak<Mutex<Directory>>,
//     path: String,
//     error: Option<ReadError>
// }

// impl Directory {
//     fn new(name: &str, parent: Weak<Mutex<Directory>>, path: &str) -> Directory {
//         Directory {
//             name: name.to_string(),
//             size: 0,
//             directories: vec![],
//             files: vec![],
//             parent: parent,
//             path: path.to_string(),
//             error: None
//         }
//     }

//     pub fn get_name(&self) -> &str {
//         &self.name
//     }

//     pub fn get_size(&self) -> u64 {
//         self.size
//     }

//     pub fn get_subdirectories(&self) -> &Vec<Arc<Mutex<Directory>>> {
//         &self.directories
//     }

//     pub fn get_files(&self) -> &Vec<File> {
//         &self.files
//     }

//     pub fn get_parent(&self) -> Weak<Mutex<Directory>> {
//         self.parent.clone()
//     }

//     pub fn get_path(&self) -> &str {
//         &self.path
//     }

//     pub fn get_error(&self) -> &Option<ReadError> {
//         &self.error
//     }

//     pub fn has_error(&self) -> bool {
//         self.error.is_some()
//     }

//     fn set_subdirectories(&mut self, subdirs: Vec<Arc<Mutex<Directory>>>) {
//         self.directories = subdirs;
//     }

//     fn set_files(&mut self, files: Vec<File>) {
//         self.files = files;
//     }

//     fn set_size(&mut self, size: u64) {
//         self.size = size;
//     }
//     fn set_error(&mut self, error: Option<ReadError>) {
//         self.error = error;
//     }
// }

// impl fmt::Display for Directory {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let sub_strings = self.directories.iter().map(|ent| ent.lock().unwrap().to_string()).collect::<Vec<String>>().join("\n");
//         let file_strings = self.files.iter().map(|ent| ent.to_string()).collect::<Vec<String>>().join("\n");
//         write!(f, "----- {} {} ------\n{}\n{}", self.name, self.size, sub_strings, file_strings)
//     }
// }

// fn path_get_file_name(path: &PathBuf) -> Option<String> {
//     if let Some(osstr) = path.file_name() {
//         match osstr.to_os_string().into_string() {
//             Ok(file_name) => Some(file_name),
//             Err(_) => None
//         }
//     }
//     else {
//         None
//     }
// }

// impl From<std::io::Error> for ReadError {
//     fn from(error: std::io::Error) -> Self {
//         ReadError::IOError(error.kind())
//     }
// }

// fn read_dir_inner(path: &PathBuf, cancel_checker: &Receiver<()>,
//                   directory: &Arc<Mutex<Directory>>, subdirectories: &mut Vec<Arc<Mutex<Directory>>>,
//                   files: &mut Vec<File>, size: &mut u64) -> Result<(), ReadError> {
//     for entry in fs::read_dir(&path)? {
//         // Normally this channel should be empty (which is an error, but one we expect)
//         // However if we try to receive and there is no error, that means the user cancelled the scan.
//         if !cancel_checker.try_recv().is_err() {
//             return Err(ReadError::OperationCancelled);
//         }
        
//         if let Ok(entry) = entry {
//             let metadata = entry.metadata()?;
//             *size += metadata.len();

//             if let Ok(name) = entry.file_name().into_string() {
//                 if metadata.is_file() {
//                     let mime = mime_guess::from_path(entry.path()).first_or_text_plain()
//                                                                   .to_string();
//                     files.push(File::new(&name, metadata.len(), &mime)); 
//                 }
//                 else if metadata.is_dir() {
//                     let dir = read_dir_impl(&entry.path(), Arc::downgrade(&directory), &cancel_checker);
//                     if let Some(e) = dir.lock().unwrap().get_error() {
//                         if let ReadError::OperationCancelled = e {
//                             return Err(ReadError::OperationCancelled);
//                         }
//                     }
//                     *size += dir.lock().unwrap().size;
//                     subdirectories.push(dir);
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// fn read_dir_impl(path: &PathBuf, parent: Weak<Mutex<Directory>>, cancel_checker: &Receiver<()>) -> Arc<Mutex<Directory>> {
//     let root_name = match path_get_file_name(&path) {
//         Some(n) => n,
//         None => "".to_string()
//     };

//     let directory = Arc::new(Mutex::new(Directory::new(&root_name, parent, &path.to_string_lossy())));
//     let mut subdirectories: Vec<Arc<Mutex<Directory>>> = Vec::new();
//     let mut files: Vec<File> = Vec::new();
//     let mut size: u64 = 0;
//     let result = read_dir_inner(&path, &cancel_checker, &directory, &mut subdirectories, &mut files, &mut size);

//     if let Ok(mut unwrapped_dir) = directory.lock() {
//         if let Err(e) = result {
//             unwrapped_dir.set_error(Some(e));
//         }
//         unwrapped_dir.set_subdirectories(subdirectories);
//         unwrapped_dir.set_files(files);
//         unwrapped_dir.set_size(size);
//     }

//     directory
// }

// pub fn read_dir(path: &PathBuf, cancel_checker: &Receiver<()>) -> Arc<Mutex<Directory>> {
//     read_dir_impl(path, Weak::new(), &cancel_checker)
// }
