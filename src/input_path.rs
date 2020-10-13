use gtk::prelude::*;
use std::thread;
use std::cell::Cell;
use std::rc::Rc;
use super::dir_walker;

pub fn show<F>(builder: gtk::Builder, when_done: F) where F: Fn(dir_walker::Directory) -> () + 'static {
    let input_path_window: gtk::Window = builder.get_object("input_path_window").unwrap();
    let window_clone = input_path_window.clone();
    let read_data = Rc::new(Cell::new(false));
    let read_data_clone = read_data.clone();
    input_path_window.connect_delete_event(move |_, _| {
        if !read_data.get() {
            gtk::main_quit();
        }
        Inhibit(false)
    });

    let file_chooser: gtk::FileChooserButton = builder.get_object("file_chooser_button").unwrap();
    let scan_progress_label: gtk::Label = builder.get_object("scan_progress_label").unwrap();
    let scan_button: gtk::Button = builder.get_object("scan_button").unwrap();
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    scan_button.connect_clicked(move |btn| {
        let sender_clone = sender.clone();
        if let Some(path) = file_chooser.get_filename() {
            btn.set_label("Cancel");
            file_chooser.set_sensitive(false);
            thread::spawn(move || {
                 sender_clone.send(dir_walker::read_dir(&path)).expect("Couldn't send data to channel");
            });
        }
    });

    receiver.attach(None, move |dir| {
        if let Ok(directory) = dir {
            scan_progress_label.set_markup(&format!("Total size: {}", directory.get_size()));
            when_done(directory);
        }

        read_data_clone.set(true);
        window_clone.close();
        glib::Continue(true)
    });
    
    input_path_window.show_all();
}
