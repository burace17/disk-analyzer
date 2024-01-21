/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::display::views::directory;

use super::traits::Constrained;
// use fallible_iterator::{convert, FallibleIterator};
use mime_guess;
use rpds::HashTrieSet;
use rpds::List;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::fs::ReadDir;
use std::hash::Hash;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Weak};
use sugar::btreeset;
use thiserror::Error;
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReadError {
    #[error("I/O error")]
    IOError(std::io::ErrorKind),
    #[error("Operation cancelled")]
    OperationCancelled,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Directory {
    name: String,
    size: u64,
    directories: List<Directory, archery::ArcTK>,
    files: List<File, archery::ArcTK>,
    parent: Option<Box<Directory>>,
    path: String,
    error: Option<ReadError>,
}

impl Directory {
    pub fn empty(name: &str, parent: Option<Box<Directory>>, path: &str) -> Directory {
        Directory {
            name: name.to_string(),
            size: 0,
            directories: List::new_with_ptr_kind(),
            files: List::new_with_ptr_kind(),
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

    pub fn get_subdirectories(&self) -> &List<Directory, archery::ArcTK> {
        &self.directories
    }

    pub fn get_files(&self) -> &List<File, archery::ArcTK> {
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

    fn set_subdirectories(&mut self, subdirs: List<Directory, archery::ArcTK>) {
        self.directories = subdirs;
    }

    fn set_files(&mut self, files: List<File, archery::ArcTK>) {
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
            // .collect::<List<String>>()
            .fold(String::new(), |string_builder, dir_string| string_builder + dir_string.as_str() + "\n");
        let file_strings = self
            .files
            .iter()
            .map(|ent| ent.to_string())
            // .collect::<List<String>>()
            // .join("\n")
            .fold(String::new(), |string_builder, dir_string| string_builder + dir_string.as_str() + "\n")
            ;
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

fn update_size_with_directory(
    entry_list: &List<DirEntry, archery::ArcTK>,
    unread_subdirectories: &List<&DirEntry, archery::ArcTK>,
    size: u64) -> u64 {
    let existing_size = entry_list
        .iter()
        .map(|directory| directory.metadata().unwrap().len())
        .sum::<u64>();
    let unread_directory_size = size * unread_subdirectories.len() as u64;
    existing_size + unread_directory_size
}

// note: using sets can allow for interesting unions, but alas these are not implemented
fn collect_files(entry_list: &List<DirEntry, archery::ArcTK>) -> List<File, archery::ArcTK> { 
    let files = entry_list
        .iter()
        .filter(|directory| directory.metadata().unwrap().is_file())
        .map(|dir_entry| {
            (
                dir_entry.path(),
                dir_entry.file_name().into_string(),
                dir_entry.metadata().unwrap().len(),
            )
        })
        .filter(|(_, filename, _)| filename.is_ok())
        .map(|(path, filename, metadata_length)| (path, filename.unwrap(), metadata_length))
        .map(|(path, filename, metadata_length)| {
            let mime = mime_guess::from_path(path)
                .first_or_text_plain()
                .to_string();
            File::new(&filename, metadata_length, &mime)
        })
        .fold(List::new_with_ptr_kind(), |mut file_list, file| {
            file_list.push_front(file);
            file_list
        })
        // .iter()
        // .map(Arc::new).collect::<List<Arc<&File>>>()
        ;
    files
}

fn collect_subdirectories(
    directory_list: &List<&DirEntry, archery::ArcTK>,
    directory_reader: impl Fn(&PathBuf) -> Directory) -> List<Directory, archery::ArcTK> {
    let read_subdirectories = directory_list
        .iter()
        .map(|subdirectory| directory_reader(&subdirectory.path()));
    let subdirectories = read_subdirectories
    .fold(List::new_with_ptr_kind(), |mut subdir_list, subdir| {
        subdir_list.push_front(subdir);
        subdir_list
    });
    subdirectories
}

fn build_directory(
    entry_list_results: List<DirEntry, archery::ArcTK>, 
    parent: &Directory, 
    cancel_checker: &Receiver<()>, 
    mut directory: Directory) -> Directory{
    let files: List<File, archery::ArcTK> = collect_files(&entry_list_results);
    let directory_list = entry_list_results.iter()
        .filter(|directory| !directory.metadata().unwrap().is_file())
        .collect::<List<&DirEntry, archery::ArcTK>>();
    let subdirectories = collect_subdirectories(&directory_list, |path| {
        return read_dir_impl(path, parent, cancel_checker);
    });
    let size = update_size_with_directory(&entry_list_results, &directory_list, 0);
    directory.set_subdirectories(subdirectories);
    directory.set_files(files);
    directory.set_size(size);
    directory
}

fn build_directories(
    entry_list_results: List<DirEntry, archery::ArcTK>, 
    mut directory: Directory, 
    parent: &Directory, 
    cancel_checker: &Receiver<()>) -> Directory {
    let metadata_error = entry_list_results
        .iter()
        .find(|suspect_dir| suspect_dir.metadata().is_err());
    match metadata_error {
        Some(err_result) => {
            let error: io::Error = err_result.metadata().err().unwrap();
            directory.set_error(Some(ReadError::from(error)));
            directory
        }
        None => build_directory(entry_list_results, parent, cancel_checker, directory)
    }
}

fn read_dir_impl(path: &PathBuf, parent: &Directory, cancel_checker: &Receiver<()>) -> Directory {
    let root_name = match path_get_file_name(&path) {
        Some(n) => n,
        None => "".to_string(),
    };
    let directory = Directory::empty(
        &root_name,
        Some(Box::from(parent.clone())),
        &path.to_string_lossy(),
    );
    let unchecked_directory_info = fs::read_dir(&path);
    match unchecked_directory_info {
        Ok(directory_info) => build_directories(directory_info
                .filter_map(Result::ok)
                .collect(), directory, parent, cancel_checker),
        Err(_e) => directory,
    }
}

pub fn read_dir(path: &PathBuf, cancel_checker: &Receiver<()>) -> Directory {
    read_dir_impl(
        path,
        &Directory::empty("empty", None, path.to_str().expect("no path provided")),
        &cancel_checker,
    )
}
use winapi::um::fileapi::GetLogicalDrives;

fn list_drives() -> HashMap<String, PathBuf> {
    let bitmask = unsafe { GetLogicalDrives() }; // DWORD
    let letter_masks = 0..26;
    let drives = letter_masks
        .map(|drive_index| (drive_index, 1 << drive_index))
        .filter(|(_, mask)| bitmask & mask != 0)
        .map(|(d_index, _)| (b'A' + d_index as u8) as char)
        .fold(Vec::new(), |mut drive_list, d_letter| {
            drive_list.push(d_letter.to_string());
            drive_list
        });
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
