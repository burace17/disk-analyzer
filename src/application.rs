/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use std::thread;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{channel, Sender};
// use super::dir_walker;
// use super::analyzer;
#![allow(unused_imports)]
use iced::widget::{container, button, column, pick_list};
use iced::{Command, Application, Theme, Element, Length, theme, Settings, Subscription, Event};
use iced::{executor, window, subscription};
use super::directory::get_computer_drives;


#[derive(Debug, Clone)]
pub enum ApplicationEvent {
    DropdownSelected,
    DriveSelected(String),
    RequestedScan,
    RequestedCancel,
    IcedEvent(iced::Event)
}
pub struct GUI {
    // model: ConfigModel,
    // file_chooser: gtk::FileChooserButton,
    // analyzer_win: Option<Component<analyzer::AnalyzerWindow>>,
    selected_drive: Option<String>
 }


     /* top level app presentation interface */
 impl Application for GUI {
     type Executor = executor::Default;
     type Flags = ();
     type Message = ApplicationEvent;
     type Theme = Theme;
 
    // __x: () = unused variable with unspecified type
    // in contrast to
    // y: int
    fn new(__flags: ()) -> (GUI, Command<ApplicationEvent>) { (GUI { selected_drive: Option::None}, Command::none()) }
    fn view(&self) -> Element<ApplicationEvent> {
        let options = get_computer_drives();
        // let options: Vec<String> = vec!["a", "b", "c"].iter().map(|&s| String::from(s)).collect();  
        let directory_list = 
            pick_list(options, self.selected_drive.clone(), ApplicationEvent::DriveSelected)
            .placeholder("Select a directory...");
        let scan_button = button("scan")
            .on_press(ApplicationEvent::RequestedScan)
            .padding(10)
            .style(theme::Button::Text);
        let cancel_button = button("cancel")
            .on_press(ApplicationEvent::RequestedCancel)
            .padding(10)
            .style(theme::Button::Text);
        let app_context = column![directory_list, scan_button, cancel_button]
            .spacing(20)
            .max_width(200);
        container(app_context)
            .height(Length::Fill)
            .center_y()
            .into()
    }
    fn title(&self) -> String { String::from("Disk Analyzer") }
    fn update(&mut self, message: ApplicationEvent) -> Command<ApplicationEvent> {
       match message {
        ApplicationEvent::DropdownSelected => { Command::none() },
        ApplicationEvent::DriveSelected(drive) => { self.selected_drive = Some(drive); Command::none() },
        ApplicationEvent::RequestedScan => { Command::none() },
        ApplicationEvent::RequestedCancel => { Command::none() },
        ApplicationEvent::IcedEvent(event) => {
            // does not work
            if let Event::Window(window::Event::CloseRequested) = event { 
                println!("test");
            }
            Command::none()
        }
       }
    }    
    fn subscription(&self) -> Subscription<ApplicationEvent> {
        subscription::events().map(ApplicationEvent::IcedEvent)
    }
}

pub fn run(settings: Settings<<GUI as iced::Application>::Flags>) -> Result<(), iced::Error> { GUI::run(settings) }
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
//  impl Widget for ConfigWindow {
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