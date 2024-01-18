// /* This Source Code Form is subject to the terms of the Mozilla Public
//  * License, v. 2.0. If a copy of the MPL was not distributed with this
//  * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use humansize::WINDOWS;
// use humansize;
use super::directory;
use crate::logic::directory::Directory;
use iter_set::symmetric_difference;
use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};
static FOLDER_ICON: &str = "folder";
static ERROR_ICON: &str = "dialog-error";

// type CellDataFunc = Box<dyn Fn(&gtk::TreeViewColumn, &gtk::CellRenderer, &gtk::TreeModel, &gtk::TreeIter) + 'static>;
#[derive(Clone, Builder, PartialEq, Eq, PartialOrd, Ord)]
pub struct DirStore {
    pub icon: String,
    pub name: String,
    pub outer_size: u64,
    pub inner_size: u64,
}

pub fn fill_list_store(dir: Directory) -> BTreeSet<DirStore> {
    let current_directory = dir.clone();
    let current_directory_size = current_directory.get_size();
    let current_sub_directories = current_directory.get_subdirectories();
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
        .collect();
    let valid_stores = current_sub_directories
        .difference(&error_dirs)
        .map(|subdir| make_store_by_icon(FOLDER_ICON, subdir))
        .collect();
    let error_stores = error_dirs
        .iter()
        .map(|subdir| make_store_by_icon(ERROR_ICON, subdir))
        .collect::<BTreeSet<DirStore>>();
    let dir_stores = error_stores
        .union(&valid_stores)
        .cloned()
        .collect::<BTreeSet<DirStore>>();
    let current_directory_files = current_directory.get_files();
    let file_stores: BTreeSet<DirStore> = current_directory_files
        .iter()
        .map(|file| DirStore {
            icon: String::from(file.get_mime()),
            name: String::from(file.get_name()),
            outer_size: current_directory_size,
            inner_size: file.get_size(),
        })
        .collect();
    let store_list = dir_stores
        .union(&file_stores)
        .cloned()
        .collect::<BTreeSet<DirStore>>();
    store_list
}
#[derive(Clone, Builder)]
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

pub struct AnalyzerModel {
    root: Arc<Mutex<directory::Directory>>,
    current: Weak<Mutex<directory::Directory>>,
}

// #[derive(Msg)]
pub enum AnalyzerMsg {
    Quit,
    RowActivated(ViewColumn),
    Up,
}

pub struct AnalyzerWindow {
    model: AnalyzerModel,
}
