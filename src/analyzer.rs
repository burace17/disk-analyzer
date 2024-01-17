// /* This Source Code Form is subject to the terms of the Mozilla Public
//  * License, v. 2.0. If a copy of the MPL was not distributed with this
//  * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use humansize::WINDOWS;
// use humansize;
use std::{sync::{Arc, Weak, Mutex}, collections::HashMap, path::PathBuf};
use crate::{directory::Directory, application::View};

use super::directory;

static FOLDER_ICON: &str = "folder";
static ERROR_ICON: &str = "dialog-error";

// type CellDataFunc = Box<dyn Fn(&gtk::TreeViewColumn, &gtk::CellRenderer, &gtk::TreeModel, &gtk::TreeIter) + 'static>;
struct DirStore {
    icon: str,
    name: str,
    outer_size: i32,
    inner_size: i32,
}

fn fill_list_store(dir: Directory) -> Vec<DirStore> {
    let current_directory = dir.lock().unwrap();
    let current_directory_size = current_directory.get_size();
    let store_list = Vec::new();
    for sub in current_directory.get_subdirectories() {
        let subdir = sub.lock().unwrap();
        if subdir.has_error() {
            store_list.append(DirStore::new(&ERROR_ICON, &subdir.get_name(), &current_directory_size, &subdir.get_size()));
        }
        else {
            store_list.append(DirStore::new(&FOLDER_ICON, &subdir.get_name(), &current_directory_size, &subdir.get_size()));
        }
    }
    for file in current_directory.get_files() {
        store_list.append(DirStore::new(&file.get_mime(), &file.get_name(), &current_directory_size, &file.get_size()));
    }
    store_list
}

pub struct ViewColumn {
    pack_start: bool,
    title: String,
    clickable: bool,
    sortable: bool,
    sort_id: Option<String>, // &'static 
    children: HashMap<String, ViewColumn>,
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
            content: None

        }
    }
}

// <R: IsA<gtk::CellRenderer>>
fn create_column(id: i32, title: &str, content: Option<String>, is_sortable: bool) -> ViewColumn
{
    let id = if is_sortable { Some(id) } else { None };
    ViewColumn::new(true, title, is_sortable, is_sortable, id, HashMap::new(), content)
    // let column = gtk::TreeViewColumn::new();

    // column.pack_start(&cell, true);
    // column.set_title(title);

    // if is_sortable {
    //     column.set_clickable(true);
    //     column.set_sort_indicator(true);
    //     column.set_sort_column_id(id);
    // }

    // if data_func.is_some() {
    //     gtk::TreeViewColumnExt::set_cell_data_func(&column, &cell, data_func);
    // }
    // else {
    //     column.add_attribute(&cell, "text", id);
    // }
    // tree.append_column(&column);
}

pub fn create_analyzer_columns(file_list: ViewColumn) -> ViewColumn {
    let icon_data_func = Box::new(|_, render, model, iter| {
        let cell = render.clone().downcast().expect("Expected renderer to be CellRenderText");
        let model_val = model.get_value(&iter, 0);
        let icon_name = model_val.get::<&str>().expect("Couldn't get icon name").expect("Couldn't get icon name");

        if icon_name == FOLDER_ICON || icon_name == ERROR_ICON {
            // cell.set_property_icon_name(Some(icon_name));
            Some(icon_name)
        }
        else {
            let icon = "foo"; //get_content_type_icon(icon_name);
            // cell.set_property_gicon(icon.as_ref());
            Some(icon.as_ref())
        }
    });
    let icon = "f";
    file_list.children.insert(String::from(""), create_column(0, "", Some(String::from(icon)), false));
    file_list.children.insert(String::from("Name"), create_column(1, "Name", None, true));

    let percentage_data_func = Box::new(|_, render, model, iter| {
        // let cell = render.clone().downcast::<gtk::CellRendererText>().expect("Expected renderer to be CellRenderText");
        let our_size = model.get_value(&iter, 3).get::<u64>()
            .expect("Couldn't get size value from tree model")
            .expect("Couldn't get size value from tree model") as f64;
        let total_size = model.get_value(&iter, 2).get::<u64>()
            .expect("Couldn't get size value from tree model")
            .expect("Couldn't get size value from tree model") as f64;

        let percentage = (our_size / total_size) * 100.0;
        let formatted = format!("{:.0}%", percentage);
        formatted
        // cell.set_property_text(Some(&formatted));
    });
    file_list.children.insert(String::from("%"), create_column(2, "%", Some(percentage_data_func), false));

    let size_data_func = Box::new(|_, render, model, iter| {
        // let cell = render.clone().downcast::<gtk::CellRendererText>().expect("Expected renderer to be CellRenderText");
        let val = model.get_value(&iter, 3).get::<u64>()
            .expect("Couldn't get size value from tree model")
            .expect("Couldn't get size value from tree model");
        let formatted_size = val.file_size(WINDOWS).unwrap();
        formatted_size
        // cell.set_property_text(Some(&formatted_size));
    });
    file_list.children.insert(String::from("Name"), create_column(3, "Size", Some(size_data_func), true));
    file_list
}

pub struct AnalyzerModel {
    root: Arc<Mutex<directory::Directory>>,
    current: Weak<Mutex<directory::Directory>>
}

// #[derive(Msg)]
pub enum AnalyzerMsg {
    Quit,
    RowActivated(ViewColumn),
    Up
}

pub struct AnalyzerWindow {
    model: AnalyzerModel,
    // window: Window,
    // list_store: gtk::ListStore,
    // sort_store: gtk::TreeModelSort,
    // header_bar: gtk::HeaderBar
}

impl AnalyzerWindow {
    fn on_row_activated(&mut self, path: PathBuf) {
        let current = self.model.current.upgrade().expect("Shouldn't be none");
        let current_unlocked = current.lock().unwrap();
        let subdirs = current_unlocked.get_subdirectories();
        let files_start_index = subdirs.len();
        let indices = self.sort_store.convert_path_to_child_path(&path)
            .expect("Sorted path does not correspond to real path").get_indices();
        if indices.len() > 0 {
            let index = indices[0] as usize;
            if index < files_start_index { // only want directories
                let new_dir = &subdirs[index];
                if new_dir.lock().unwrap().has_error() {
                    let msg = format!("Could not read directory contents");
                    // let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
                    //                                           gtk::ButtonsType::Ok, &msg);
                    // message_box.run();
                    // message_box.hide();
                }
                else {
                    self.list_store.clear();
                    self.list_store = fill_list_store(&new_dir);
                    self.header_bar.set_subtitle(Some(new_dir.lock().unwrap().get_path()));
                    self.model.current = Arc::downgrade(&new_dir);
                }
            }
        }
    }

    fn on_up_clicked(&mut self) {
        let current = self.model.current.upgrade().expect("Current dir shouldn't be none");
        let parent_ptr = current.lock().unwrap().get_parent();
        if let Some(parent) = parent_ptr.upgrade() {
            self.list_store.clear();
            self.list_store = fill_list_store(&parent);
            self.header_bar.set_subtitle(Some(parent.lock().unwrap().get_path()));
            self.model.current = Arc::downgrade(&parent);
        }
    }
}


// impl Update for AnalyzerWindow {
//     type Model = AnalyzerModel;
//     type ModelParam = Arc<Mutex<directory::Directory>>;
//     type Msg = AnalyzerMsg;

//     fn model(_: &Relm<Self>, dir: Self::ModelParam) -> AnalyzerModel {
//         let current_ref = Arc::downgrade(&dir);
//         AnalyzerModel {
//             root: dir,
//             current: current_ref
//         }
//     }

//     fn update(&mut self, event: AnalyzerMsg) {
//         match event {
//             AnalyzerMsg::Quit => gtk::main_quit(),
//             AnalyzerMsg::RowActivated(path) => self.on_row_activated(path),
//             AnalyzerMsg::Up => self.on_up_clicked()
//         }
//     }
// }

// impl Widget for AnalyzerWindow {
//     // type Root = Window;

//     fn root(&self) -> Self::Root {
//         self.window.clone()
//     }

//     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
//         let file_list = gtk::TreeView::new();
//         create_analyzer_columns(&file_list);

//         let file_model = gtk::ListStore::new(&[String::static_type(), String::static_type(), u64::static_type(), u64::static_type()]);
//         let sortable_store = gtk::TreeModelSort::new(&file_model);
//         sortable_store.set_sort_column_id(gtk::SortColumn::Index(3), gtk::SortType::Descending);
//         file_list.set_model(Some(&sortable_store));
//         fill_list_store(&file_model, &model.root);

//         let viewport = gtk::Viewport::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
//         viewport.add(&file_list);
        
//         let scrolled = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
//         scrolled.add(&viewport);
//         scrolled.set_vexpand(true);

//         let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//         vbox.add(&scrolled);

//         let header_bar = gtk::HeaderBar::new();
//         let up_button = gtk::Button::from_icon_name(Some("go-up"), gtk::IconSize::Menu);
//         up_button.set_tooltip_text(Some("Up"));
//         header_bar.set_title(Some("Disk Analyzer"));
//         header_bar.set_subtitle(Some(model.root.lock().unwrap().get_path()));
//         header_bar.set_show_close_button(true);
//         header_bar.pack_start(&up_button);
        
//         let window = gtk::Window::new(WindowType::Toplevel);
//         window.add(&vbox);
//         window.set_position(gtk::WindowPosition::Center);
//         window.resize(800, 600);
//         window.set_titlebar(Some(&header_bar));

//         connect!(relm, window, connect_delete_event(_, _), return (Some(AnalyzerMsg::Quit), Inhibit(false)));
//         connect!(relm, up_button, connect_clicked(_), AnalyzerMsg::Up);
//         connect!(relm, file_list, connect_row_activated(_, path, _), AnalyzerMsg::RowActivated(path.clone()));

//         AnalyzerWindow {
//             model,
//             window,
//             list_store: file_model,
//             sort_store: sortable_store,
//             header_bar: header_bar
//         }
//     }
// }