use gtk::prelude::*;
use humansize::{FileSize, file_size_opts as options};
use super::dir_walker;

type CellDataFunc = Box<dyn Fn(&gtk::TreeViewColumn, &gtk::CellRenderer, &gtk::TreeModel, &gtk::TreeIter) + 'static>;

fn setup_model(directory: dir_walker::Directory, _parent: Option<&dir_walker::Directory>,
               file_list: gtk::TreeView,
               model: gtk::ListStore,
               sorted_store: gtk::TreeModelSort,
               builder: gtk::Builder) {
    model.clear();
    for dir in directory.get_subdirectories() {
        model.insert_with_values(None, &[0, 1], &[&dir.get_name(), &dir.get_size()]);
    }
    for file in directory.get_files() {
        model.insert_with_values(None, &[0, 1], &[&file.get_name(), &file.get_size()]);
    }

    file_list.connect_row_activated(move |_, path, _| {
        let subdirectories = directory.get_subdirectories();
        let files_start_index = subdirectories.len();
        let indices = sorted_store.convert_path_to_child_path(&path)
            .expect("Sorted path does not correspond to real path").get_indices();
        if indices.len() > 0 {
            let index = indices[0] as usize;
            if index < files_start_index {
                // this is a directory.
                let current_list: gtk::TreeView = builder.get_object("file_list").unwrap();
                let new_dir = subdirectories[index].clone();
                setup_model(new_dir, Some(&directory), current_list, model.clone(), sorted_store.clone(), builder.clone());
            }
        }
    });
}

pub fn show(builder: gtk::Builder, directory: dir_walker::Directory) {
    let analysis_window: gtk::Window = builder.get_object("analysis_window").unwrap();
    analysis_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

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

    let file_list: gtk::TreeView = builder.get_object("file_list").unwrap();
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

    setup_model(directory, None, file_list, file_model, sortable_store, builder);
    analysis_window.show_all();
}
