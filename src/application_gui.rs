/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use std::thread;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{channel, Sender};
use iced::widget::pick_list;
// use super::dir_walker;
// use super::analyzer;
use iced::{Command, Application, Theme, Element, Length};
use iced::executor;
// pub struct ConfigModel {
//     path: Option<std::path::PathBuf>,
//     relm: Relm<ConfigWindow>
// }
// #[derive(Msg)]
// pub enum ConfigMsg {
//     Quit,
//     GotPath(Option<std::path::PathBuf>),
//     StartScan,
//     GotResults(Arc<Mutex<dir_walker::Directory>>),
//     CancelScan
// }
// impl ConfigWindow {
//     fn reset_ui(&self) {
//         self.scan_button.set_label("Load");
//         self.scan_button.set_sensitive(true);
//         self.file_chooser.set_sensitive(true);
//         self.cancel_button.set_sensitive(false);
//     }
//     fn on_scan_start(&mut self) {
//         if let Some(file_path) = self.model.path.clone() {
//             let stream = self.model.relm.stream().clone();
//             let (_, sender) = Channel::new(move |dir| {
//                 stream.emit(ConfigMsg::GotResults(dir));
//             });
//             let (send, recv) = channel();
//             self.cancel_sender = Some(send);
//             self.scan_button.set_label("Reading...");
//             self.scan_button.set_sensitive(false);
//             self.file_chooser.set_sensitive(false);
//             self.cancel_button.set_sensitive(true);
//             thread::spawn(move || {
//                 let dir = dir_walker::read_dir(&file_path, &recv);
//                 sender.send(dir).expect("Couldn't send message");
//             });
//         }
//     }
//     fn on_scan_complete(&mut self, dir: Arc<Mutex<dir_walker::Directory>>) {
//         self.cancel_sender = None;
//         let dir_clone = dir.clone();
//         let error = dir.lock().unwrap().get_error().clone();
//         match error {
//             None => {
//                 self.window.hide();
//                 let analyzer_win = init::<analyzer::AnalyzerWindow>(dir_clone).expect("Couldn't init");
//                 analyzer_win.widget().show_all();
//                 self.analyzer_win = Some(analyzer_win);
//             },
//             Some(e) => match e {
//                 dir_walker::ReadError::IOError(_) => {
//                     let msg = "Could not read directory contents";
//                     let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
//                                                               gtk::ButtonsType::Ok, &msg);
//                     message_box.run();
//                     message_box.hide();
//                     self.reset_ui();
//                 },
//                 dir_walker::ReadError::OperationCancelled => self.reset_ui()
//             }
//         }
//     }
//     fn on_scan_cancel(&self) {
//         self.cancel_button.set_sensitive(false);
//         if let Some(tracker) = &self.cancel_sender {
//             tracker.send(()).unwrap();
//         }
//     }
// }
// impl Update for ConfigWindow {
//     type Model = ConfigModel;
//     type ModelParam = ();
//     type Msg = ConfigMsg;    
//     fn model(relm: &Relm<Self>, _: ()) -> ConfigModel {
//         ConfigModel {
//             path: None,
//             relm: relm.clone()
//         }
//     }
//     fn update(&mut self, event: ConfigMsg) {
//         match event {
//             ConfigMsg::Quit => gtk::main_quit(),
//             ConfigMsg::GotPath(path) => self.model.path = path,
//             ConfigMsg::StartScan => self.on_scan_start(),
//             ConfigMsg::GotResults(result) => self.on_scan_complete(result),
//             ConfigMsg::CancelScan => self.on_scan_cancel()
//         }
//     }
// }
// impl Widget for ConfigWindow {
//     type Root = Window;
//     fn root(&self) -> Self::Root {
//         self.window.clone()
//     }
//     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
//         let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//         let file_chooser = gtk::FileChooserButton::new("Choose directory", gtk::FileChooserAction::SelectFolder);
//         let scan_button = gtk::Button::new();
//         scan_button.set_label("Scan");
//         let cancel_button = gtk::Button::new();
//         cancel_button.set_label("Cancel");
//         cancel_button.set_sensitive(false);
//         vbox.add(&file_chooser);
//         vbox.add(&scan_button);
//         vbox.add(&cancel_button);
//         vbox.set_spacing(10);
//         let window = gtk::Window::new(WindowType::Toplevel);
//         window.set_title("Choose a directory to scan");
//         window.add(&vbox);
//         window.set_position(gtk::WindowPosition::Center);
//         window.resize(300, 75);
//         window.show_all();
//         connect!(relm, scan_button, connect_clicked(_), ConfigMsg::StartScan);
//         connect!(relm, cancel_button, connect_clicked(_), ConfigMsg::CancelScan);
//         connect!(relm, file_chooser, connect_file_set(btn), ConfigMsg::GotPath(btn.get_filename()));
//         connect!(relm, window, connect_delete_event(_, _), return (Some(ConfigMsg::Quit), Inhibit(false)));
//         ConfigWindow {
//             model,
//             window,
//             file_chooser,
//             scan_button,
//             analyzer_win: None,
//             cancel_sender: None,
//             cancel_button
//         }
//     }
// }
#[derive(Debug, Clone, Copy)]
enum application_events {
    DropdownSelected,
    DirectorySelected,
    RequestedScan,
    RequestedCancel
}
struct analyzer_gui {
    // model: ConfigModel,
    // window: Window,
    // directory_list: pick_list,
    // // file_chooser: gtk::FileChooserButton,
    // scan_button: Button,
    // analyzer_win: Option<Component<analyzer::AnalyzerWindow>>,
    // // cancel_sender: Option<Sender<()>>,
    // cancel_button: gtk::Button
 }

 impl Application for analyzer_gui {
     type Executor = executor::Default;
     type Flags = ();
     type Message = application_events;
     type Theme = Theme;
 
    // __x: () = unused variable with unspecified type
    // in contrast to
    // y: int
    fn new(__flags: ()) -> (analyzer_gui, Command<application_events>) {
       (analyzer_gui {}, Command::none())
    }
    fn view(&self) -> Element<application_events> {
        container(app_context).height(Length::Fill)
            .center_y().into()
    }
    fn title(&self) -> String {
       String::from("Disk Analyzer")
   }
    fn update(&mut self, message: application_events) -> Command<Self::Message> {
       match message {
        application_events::DropdownSelected => { Command::none() },
        application_events::DirectorySelected => { Command::none() },
        application_events::RequestedScan => { Command::none() },
        application_events::RequestedCancel => { Command::none() },
       }
    }
 }