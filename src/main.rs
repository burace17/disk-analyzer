extern crate gtk;
extern crate glib;
extern crate gio;

use gtk::prelude::*;
use std::thread;
mod dir_walker;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("input_path_window").unwrap();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let file_chooser: gtk::FileChooserButton = builder.get_object("file_chooser_button").unwrap();
    let file_chooser_clone = file_chooser.clone();
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
        }
        scan_button.set_label("Scan");
        file_chooser_clone.set_sensitive(true);
        glib::Continue(true)
    });
    
    window.show_all();
    
    gtk::main();
}
