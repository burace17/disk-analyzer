/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use mime_guess;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Weak};
use sugar::btreeset;
use thiserror::Error;
use fallible_iterator::{convert, FallibleIterator};
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReadError {
    #[error("I/O error")]
    IOError(std::io::ErrorKind),
    #[error("Operation cancelled")]
    OperationCancelled,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct File {
    name: String,
    size: u64,
    mime: String,
}

impl File {
    fn new(name: &str, size: u64, mime: &str) -> File {
        File {
            name: name.to_string(),
            size: size,
            mime: mime.to_string(),
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

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Directory {
    name: String,
    size: u64,
    directories: BTreeSet<Directory>,
    files: BTreeSet<File>,
    parent: Option<Box<Directory>>,
    path: String,
    error: Option<ReadError>,
}

impl Directory {
    pub fn new(name: &str, parent: Option<Box<Directory>>, path: &str) -> Directory {
        Directory {
            name: name.to_string(),
            size: 0,
            directories: btreeset![],
            files: btreeset![],
            parent: parent,
            path: path.to_string(),
            error: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_subdirectories(&self) -> &BTreeSet<Directory> {
        &self.directories
    }

    pub fn get_files(&self) -> &BTreeSet<File> {
        &self.files
    }

    // pub fn get_parent(&self) -> Directory {
    //     let boxed_dir = self.parent.unwrap();
    //     boxed_dir.clone()
    // }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_error(&self) -> &Option<ReadError> {
        &self.error
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    fn set_subdirectories(&mut self, subdirs: BTreeSet<Directory>) {
        self.directories = subdirs;
    }

    fn set_files(&mut self, files: BTreeSet<File>) {
        self.files = files;
    }

    fn set_size(&mut self, size: u64) {
        self.size = size;
    }
    fn set_error(&mut self, error: Option<ReadError>) {
        self.error = error;
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sub_strings = self
            .directories
            .iter()
            .map(|ent| ent.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let file_strings = self
            .files
            .iter()
            .map(|ent| ent.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "----- {} {} ------\n{}\n{}",
            self.name, self.size, sub_strings, file_strings
        )
    }
}

fn path_get_file_name(path: &PathBuf) -> Option<String> {
    let path_file_name = path
        .file_name()
        .ok_or("Path has no filename")
        .map(|os_string| {
            os_string
                .to_os_string()
                .into_string()
                .unwrap_or(String::from("Could not convert to os string"))
        });
    path_file_name.ok()
}

impl From<std::io::Error> for ReadError {
    fn from(error: std::io::Error) -> Self {
        ReadError::IOError(error.kind())
    }
}

fn add_files() {
    let filenames = existing_directories.iter()
    .map(|dir_entry| dir_entry.file_name().into_string())
    .filter_map(|arg0: Result<String, OsString>| Result::ok(arg0))
    ;
  let directory_files: Vec<&DirEntry> = existing_directories.iter().filter(|directory| directory.metadata().unwrap().is_file()).collect();
  let read_files = directory_files.iter().map(|dir_entry| {
    let mime = mime_guess::from_path(dir_entry.path()).first_or_text_plain().to_string();
    File::new(&dir_entry.file_name().into_string().unwrap(), dir_entry.metadata().unwrap().len(), &mime)
  }); //todo: better, since filenames is only used here
  read_files.fold(files, |file_list, file| {file_list.insert(file); file_list});
}

fn add_subdirectories(existing_directories: DirectoriesWithMetadata) {
    let unread_subdirectories: Vec<&DirEntry> = existing_directories.iter().filter(|directory| !directory.metadata().unwrap().is_file()).collect();
    let total_subdirectories = unread_subdirectories.iter()
      .map(|subdirectory| read_dir_impl(&subdirectory.path(), directory, cancel_checker));
    total_subdirectories.fold(subdirectories, |subdir_list, subdir| {subdir_list.insert(subdir); subdir_list});
    Ok(())
}

fn update_size_with_directory() {
    *size += existing_directories.access.iter().map(|directory| directory.metadata().unwrap().len()).sum::<u64>();
    *size += *size * unread_subdirectories.len() as u64;
}

fn slurp_directories() {

}

fn read_dir_inner(
    path: &PathBuf,
    cancel_checker: &Receiver<()>,
    directory: &Directory,
    subdirectories: &mut BTreeSet<Directory>,
    files: &mut BTreeSet<File>,
    size: &mut u64) -> Result<(), ReadError> {
    let directory_info = fs::read_dir(&path)?;
    let existing_directories: Vec<DirEntry> = directory_info.filter_map(Result::ok).collect();
    let valid_directories = existing_directories.iter().map(DirectoriesWithMetadata::constrain);
    let metadata_error = valid_directories.find(Result::is_err);    
    if metadata_error.is_some() { // todo: better
      metadata_error.unwrap().metadata()?;
    }
    OK(())
    // slurp_directories()
}

fn read_dir_impl(path: &PathBuf, parent: &Directory, cancel_checker: &Receiver<()>) -> Directory {
    let root_name = match path_get_file_name(&path) {
        Some(n) => n,
        None => "".to_string(),
    };

    let mut directory = Directory::new(
        &root_name,
        Some(Box::from(parent.clone())),
        &path.to_string_lossy(),
    ); //Arc::new(Mutex::new());
    let mut subdirectories: BTreeSet<Directory> = BTreeSet::new();
    let mut files: BTreeSet<File> = BTreeSet::new();
    let mut size: u64 = 0;
    let result = read_dir_inner(
        &path,
        &cancel_checker,
        &directory,
        &mut subdirectories,
        &mut files,
        &mut size,
    );

    // if let Ok(mut unwrapped_dir) = directory.lock() {
    // if let Err(e) = result {
    //     unwrapped_dir.set_error(Some(e));
    // }
    directory.set_subdirectories(subdirectories);
    directory.set_files(files);
    directory.set_size(size);
    // }

    directory
}

pub fn read_dir(path: &PathBuf, cancel_checker: &Receiver<()>) -> Directory {
    read_dir_impl(
        path,
        &Directory::new("empty", None, path.to_str().expect("no path provided")),
        &cancel_checker,
    )
}
use winapi::um::fileapi::GetLogicalDrives;

//todo bonus: pass in DWORD
fn list_drives() -> HashMap<String, PathBuf> {
    let bitmask = unsafe { GetLogicalDrives() }; // DWORD
    let letter_masks = 0..26;
    let drives = letter_masks
      .map(|drive_index| (drive_index, 1 << drive_index))
      .filter(|(_, mask)| bitmask & mask != 0)
      .map(|(d_index, _)| (b'A' + d_index as u8) as char)
      .fold(Vec::new(), |mut drive_list, d_letter| {drive_list.push(d_letter.to_string()); drive_list});
    let letter_with_path: Vec<(String, PathBuf)> = drives
        .iter()
        .map(|s: &String| {
            let drive_as_path = PathBuf::from(s.to_string() + ":");
            let pair = (s.clone(), drive_as_path);
            pair
        })
        .collect();
    let letter_to_path = letter_with_path.into_iter().collect();
    letter_to_path
}

pub fn get_computer_drives() -> HashMap<String, PathBuf> {
    list_drives()
}
