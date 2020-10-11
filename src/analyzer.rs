use gtk::prelude::*;
use super::dir_walker;
pub fn show(builder: &gtk::Builder, directory: dir_walker::Directory) {
    let analysis_window: gtk::Window = builder.get_object("analysis_window").unwrap();
    analysis_window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    analysis_window.show_all();
}
