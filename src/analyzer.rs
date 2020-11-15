use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use humansize::{FileSize, file_size_opts as options};
use std::sync::{Arc, Weak};
use super::dir_walker;

type CellDataFunc = Box<dyn Fn(&gtk::TreeViewColumn, &gtk::CellRenderer, &gtk::TreeModel, &gtk::TreeIter) + 'static>;

pub struct AnalyzerModel {
    root: Arc<dir_walker::Directory>,
    current: Weak<dir_walker::Directory>
}

#[derive(Msg)]
pub enum AnalyzerMsg {
    Quit,
    RowActivated(gtk::TreePath)
}

pub struct AnalyzerWindow {
    model: AnalyzerModel,
    window: Window,
    list_store: gtk::ListStore,
    sort_store: gtk::TreeModelSort
}

fn fill_list_store(store: &gtk::ListStore, dir: &dir_walker::Directory) {
    for dir in dir.get_subdirectories() {
        store.insert_with_values(None, &[0, 1], &[&dir.get_name(), &dir.get_size()]);
    }
    for file in dir.get_files() {
        store.insert_with_values(None, &[0, 1], &[&file.get_name(), &file.get_size()]);
    }
}

impl Update for AnalyzerWindow {
    type Model = AnalyzerModel;
    type ModelParam = dir_walker::Directory;
    type Msg = AnalyzerMsg;

    fn model(_: &Relm<Self>, dir: Self::ModelParam) -> AnalyzerModel {
        let dir_ptr = Arc::new(dir);
        let current_ref = Arc::downgrade(&dir_ptr);
        AnalyzerModel {
            root: dir_ptr,
            current: current_ref
        }
    }

    fn update(&mut self, event: AnalyzerMsg) {
        match event {
            AnalyzerMsg::Quit => gtk::main_quit(),
            AnalyzerMsg::RowActivated(path) => {
                let current = self.model.current.upgrade().expect("Shouldn't be none");
                let subdirs = current.get_subdirectories();
                let files_start_index = subdirs.len();
                let indices = self.sort_store.convert_path_to_child_path(&path)
                    .expect("Sorted path does not correspond to real path").get_indices();
                if indices.len() > 0 {
                    let index = indices[0] as usize;
                    if index < files_start_index { // only want directories
                        self.list_store.clear();
                        let new_dir = &subdirs[index];
                        fill_list_store(&self.list_store, &new_dir);
                        self.model.current = Arc::downgrade(&new_dir);
                    }
                }
            }
        }
    }
}

impl Widget for AnalyzerWindow {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let file_list = gtk::TreeView::new();
        fn append_column(tree: &gtk::TreeView, id: i32, title: &str, data_func: Option<CellDataFunc>)
        {
            let column = gtk::TreeViewColumn::new();
            let cell = gtk::CellRendererText::new();

            column.pack_start(&cell, true);
            column.set_title(title);
            column.set_clickable(true);
            column.set_sort_indicator(true);
            column.set_sort_column_id(id);

            if data_func.is_some() {
                gtk::TreeViewColumnExt::set_cell_data_func(&column, &cell, data_func);
            }
            else {
                column.add_attribute(&cell, "text", id);
            }
            tree.append_column(&column);
        }

        append_column(&file_list, 0, "Name", None);
        let cell_data_func: CellDataFunc = Box::new(|_, render, model, iter| {
            let cell = render.clone().downcast::<gtk::CellRendererText>().expect("Expected renderer to be CellRenderText");
            let val = model.get_value(&iter, 1).get::<u64>()
                .expect("Couldn't get size value from tree model")
                .expect("Couldn't get size value from tree model");
            let formatted_size = val.file_size(options::CONVENTIONAL).unwrap();
            cell.set_property_text(Some(&formatted_size));
        });
        append_column(&file_list, 1, "Size", Some(cell_data_func));

        let file_model = gtk::ListStore::new(&[String::static_type(), u64::static_type()]);
        let sortable_store = gtk::TreeModelSort::new(&file_model);
        sortable_store.set_sort_column_id(gtk::SortColumn::Index(1), gtk::SortType::Descending);
        file_list.set_model(Some(&sortable_store));
        fill_list_store(&file_model, &model.root);

        let viewport = gtk::Viewport::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        viewport.add(&file_list);
        
        let scrolled = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        scrolled.add(&viewport);
        
        let window = gtk::Window::new(WindowType::Toplevel);
        window.add(&scrolled);
        window.set_position(gtk::WindowPosition::Center);
        window.resize(800, 600);
        connect!(relm, window, connect_delete_event(_, _), return (Some(AnalyzerMsg::Quit), Inhibit(false)));
        connect!(relm, file_list, connect_row_activated(_, path, _), AnalyzerMsg::RowActivated(path.clone()));

        AnalyzerWindow {
            model,
            window,
            list_store: file_model,
            sort_store: sortable_store
        }
    }
}
