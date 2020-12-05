use gtk::{Window, Inhibit, WindowType};
use gtk::prelude::*;
use relm::{connect, Channel, Relm, Update, Widget, Component, init};
use relm_derive::Msg;
use std::thread;
use std::sync::{Arc, Mutex};
use super::dir_walker;
use super::analyzer;

pub struct ConfigModel {
    path: Option<std::path::PathBuf>,
    relm: Relm<ConfigWindow>
}

#[derive(Msg)]
pub enum ConfigMsg {
    Quit,
    GotPath(Option<std::path::PathBuf>),
    StartScan,
    GotResults(std::io::Result<Arc<Mutex<dir_walker::Directory>>>)
}

pub struct ConfigWindow {
    model: ConfigModel,
    window: Window,
    file_chooser: gtk::FileChooserButton,
    load_button: gtk::Button,
    analyzer_win: Option<Component<analyzer::AnalyzerWindow>>
}

impl ConfigWindow {
    fn on_scan_start(&self) {
        if let Some(file_path) = self.model.path.clone() {
            let stream = self.model.relm.stream().clone();
            let (_, sender) = Channel::new(move |dir| {
                stream.emit(ConfigMsg::GotResults(dir));
            });

            // In the future this should just turn into a cancel button.
            self.load_button.set_label("Reading...");
            self.load_button.set_sensitive(false);
            self.file_chooser.set_sensitive(false);

            thread::spawn(move || {
                let dir = dir_walker::read_dir(&file_path);
                sender.send(dir).expect("Couldn't send message");
            });
        }
    }

    fn on_scan_complete(&mut self, result: std::io::Result<Arc<Mutex<dir_walker::Directory>>>) {
        match result {
            Ok(dir) => {
                self.window.hide();
                let analyzer_win = init::<analyzer::AnalyzerWindow>(dir).expect("Couldn't init");
                analyzer_win.widget().show_all();

                self.analyzer_win = Some(analyzer_win);
            },
            Err(e) => {
                let msg = format!("Could not read directory contents: {}", e);
                let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
                                                          gtk::ButtonsType::Ok, &msg);
                message_box.run();
                message_box.hide();

                self.load_button.set_label("Load");
                self.load_button.set_sensitive(true);
                self.file_chooser.set_sensitive(true);
            }
        }
    }
}

impl Update for ConfigWindow {
    type Model = ConfigModel;
    type ModelParam = ();
    type Msg = ConfigMsg;
    
    fn model(relm: &Relm<Self>, _: ()) -> ConfigModel {
        ConfigModel {
            path: None,
            relm: relm.clone()
        }
    }

    fn update(&mut self, event: ConfigMsg) {
        match event {
            ConfigMsg::Quit => gtk::main_quit(),
            ConfigMsg::GotPath(path) => self.model.path = path,
            ConfigMsg::StartScan => self.on_scan_start(),
            ConfigMsg::GotResults(result) => self.on_scan_complete(result)
        }
    }
}

impl Widget for ConfigWindow {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let file_chooser = gtk::FileChooserButton::new("Choose directory", gtk::FileChooserAction::SelectFolder);
        let button = gtk::Button::new();
        button.set_label("Scan");

        vbox.add(&file_chooser);
        vbox.add(&button);
        vbox.set_spacing(10);

        let window = gtk::Window::new(WindowType::Toplevel);
        window.set_title("Choose a directory to scan");
        window.add(&vbox);
        window.set_position(gtk::WindowPosition::Center);
        window.resize(300, 75);
        window.show_all();

        connect!(relm, button, connect_clicked(_), ConfigMsg::StartScan);
        connect!(relm, file_chooser, connect_file_set(btn), ConfigMsg::GotPath(btn.get_filename()));
        connect!(relm, window, connect_delete_event(_, _), return (Some(ConfigMsg::Quit), Inhibit(false)));

        ConfigWindow {
            model: model,
            window: window,
            file_chooser: file_chooser,
            load_button: button,
            analyzer_win: None
        }
    }
}
