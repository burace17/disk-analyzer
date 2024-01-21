// /* This Source Code Form is subject to the terms of the Mozilla Public
//  * License, v. 2.0. If a copy of the MPL was not distributed with this
//  * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use humansize::WINDOWS;
use rpds::HashTrieSet;
// use humansize;
use super::directory;
use crate::logic::directory::directory::Directory;
// use iter_set::symmetric_difference;
use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};
use archery::ArcTK;
static FOLDER_ICON: &str = "folder";
static ERROR_ICON: &str = "dialog-error";

// type CellDataFunc = Box<dyn Fn(&gtk::TreeViewColumn, &gtk::CellRenderer, &gtk::TreeModel, &gtk::TreeIter) + 'static>;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirStore {
    pub icon: String,
    pub name: String,
    pub outer_size: u64,
    pub inner_size: u64,
}

pub fn fill_list_store(dir: &Directory) -> HashTrieSet<DirStore, ArcTK> {
    let current_directory = dir; //.clone();
    let current_directory_size = current_directory.get_size();
    let current_sub_directories = current_directory
        .get_subdirectories()
        .iter()
        .cloned()
        .collect::<HashTrieSet<Directory, ArcTK>>()
        
        ;
    let make_store_by_icon = |icon: &str, subdir: &Directory| -> DirStore {
        DirStore {
            icon: String::from(icon),
            name: String::from(subdir.get_name()),
            outer_size: current_directory_size,
            inner_size: subdir.get_size(),
        }
    };
    let error_dirs = current_sub_directories
        .iter()
        .filter(|subdir| subdir.has_error())
        .cloned()
        .collect::<HashTrieSet<Directory, ArcTK>>();
    let valid_stores = current_sub_directories.iter()
        .filter(|subdir| !error_dirs.contains(subdir))
        // .difference(&error_dirs)
        .map(|subdir| make_store_by_icon(FOLDER_ICON, subdir))
        .collect::<HashTrieSet<DirStore, ArcTK>>();
    let error_stores = error_dirs
        .iter()
        .map(|subdir| make_store_by_icon(ERROR_ICON, subdir))
        .collect::<HashTrieSet<DirStore, ArcTK>>();
    let dir_stores = valid_stores.iter()
        .fold(error_stores, |dir_stores, valid_store| {
            dir_stores.insert(valid_store.clone())
        })
        // .iter()
        // .cloned()
        // .collect::<HashTrieSet<DirStore>>()
        ;
    let current_directory_files = current_directory.get_files();
    let file_stores: HashTrieSet<DirStore, ArcTK> = current_directory_files
        .iter()
        .map(|file| DirStore {
            icon: String::from(file.get_mime()),
            name: String::from(file.get_name()),
            outer_size: current_directory_size,
            inner_size: file.get_size(),
        })
        .collect();
    let store_list = dir_stores.iter()
        .fold(file_stores, |store_list, dir_store| store_list.insert(dir_store.clone()))
        // .union(&file_stores)
        // .cloned()
        // .collect::<HashTrieSet<DirStore>>()
        ;
    store_list
}
#[derive(Clone)]
pub struct ViewColumn {
    pack_start: bool,
    title: String,
    clickable: bool,
    sortable: bool,
    sort_id: Option<i32>, // &'static
    pub children: HashMap<String, ViewColumn>,
    content: Option<String>,
}

impl ViewColumn {
    pub fn default_butt_title(title: String) -> Self {
        ViewColumn {
            title: title,
            ..Default::default()
        }
    }
}

impl Default for ViewColumn {
    fn default() -> Self {
        ViewColumn {
            pack_start: false,
            title: String::from("blair is a weeb"), // if i used str, title could not own the value, &'static str if i know str at compile time
            clickable: false,
            sortable: false,
            sort_id: None,
            children: HashMap::new(),
            content: None,
        }
    }
}

// <R: IsA<gtk::CellRenderer>>
fn create_column(id: i32, title: &str, content: Option<String>, is_sortable: bool) -> ViewColumn {
    let id = if is_sortable { Some(id) } else { None };
    ViewColumn {
        pack_start: true,
        title: String::from(title),
        clickable: is_sortable,
        sortable: is_sortable,
        sort_id: id,
        children: HashMap::new(),
        content: content,
    }
}

pub fn create_analyzer_columns(mut file_list: ViewColumn) -> ViewColumn {
    let icon = "f";
    file_list.children.insert(
        String::from("Icon"),
        create_column(0, "Icon", Some(String::from(icon)), false),
    );
    file_list
        .children
        .insert(String::from("Name"), create_column(1, "Name", None, true));
    let percentage_data_func = String::from("10");
    file_list.children.insert(
        String::from("%"),
        create_column(2, "%", Some(percentage_data_func), false),
    );
    let size_data_func = String::from("69");
    file_list.children.insert(
        String::from("Size"),
        create_column(3, "Size", Some(size_data_func), true),
    );
    file_list
}

// pub struct AnalyzerModel {
//     root: Arc<Mutex<Directory>>,
//     current: Weak<Mutex<Directory>>,
// }

// #[derive(Msg)]
// pub enum AnalyzerMsg {
//     Quit,
//     RowActivated(ViewColumn),
//     Up,
// }

// pub struct AnalyzerWindow {
//     model: AnalyzerModel,
// }
