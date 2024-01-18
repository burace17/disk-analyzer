/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use std::thread;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{channel, Sender};
// use super::dir_walker;
#![allow(unused_imports)]
use std::collections::HashMap;
use std::io::Error;
use std::iter::Scan;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

use super::directory;
use crate::analyzer::{self, ViewColumn};
use crate::directory::Directory;
use crate::events::handlers;
use iced::widget::button::StyleSheet;
use iced::widget::{button, column, container, pick_list, row, text, Container, Text, Column, Row};
use iced::{executor, subscription, window, Renderer};
use iced::{theme, Application, Command, Element, Event, Length, Settings, Subscription, Theme};

#[derive(Default)]
pub struct GUI {
    view: View,
    dir: Directory,
    scan_finished: bool,
    cancel_sender: Option<Sender<()>>,
    paths: HashMap<String, PathBuf>,
    scanning: bool,
    pressed_cancel: bool,
    selected_drive: Option<String>,
}
// use super::events::handlers::on_scan_start;

/* top level app presentation interface */
impl Application for GUI {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ApplicationEvent;
    type Theme = Theme;
    // todo: where default come from and what it do?
    // __x: () = unused variable with unspecified type
    // in contrast to
    // y: int
    fn new(__flags: ()) -> (GUI, Command<ApplicationEvent>) {
        (
            GUI {
                view: View::Start,
                scan_finished: false,
                cancel_sender: None,
                dir: Directory::default(),
                paths: directory::get_computer_drives(),
                scanning: false,
                pressed_cancel: false,
                selected_drive: None,
            },
            Command::none(),
        )
    }
    fn view(&self) -> Element<ApplicationEvent> {
        match self.view {
            View::DirectoryDisplay => {
                let file_list = ViewColumn::default_butt_title(String::from("root"));
                let file_columns = analyzer::create_analyzer_columns(file_list);
                let header_view: Row<'_, ApplicationEvent> = row![];
                let header_columns = file_columns.children.iter().fold(header_view, |acc, (k, v)| {
                  acc.push(text(k))
                });
                let dir = &self.dir;
                let dir_clone = dir.clone();
                let directory_content = analyzer::fill_list_store(dir_clone);
                let directory_list = directory_content.iter().map(|dir_store| {
                  let icon = "f";
                  let percent = ((dir_store.inner_size % dir_store.outer_size) * 100).to_string() + "%";

                  let file_row: Row<'_,  ApplicationEvent> = row![text(icon), text(dir_store.name.clone()), text(percent), text(dir_store.inner_size)];
                  file_row
                });
                let directory_column = directory_list.fold(column![], |column, row| {
                  column.push(row)
                });
                let directory_display = column![header_columns, directory_column];
                container(directory_display)
                    .height(Length::Fill)
                    .center_y()
                    .into()                
            }
            View::Start => {
                let drives_as_strings: Vec<String> = self.paths.keys().cloned().collect();
                let directory_list = pick_list(
                    drives_as_strings,
                    self.selected_drive.clone(),
                    ApplicationEvent::DriveSelected,
                )
                .placeholder("Select a directory...");
                let mut scan_button = button("scan").padding(10).style(theme::Button::Primary);
                let mut cancel_button = button("cancel").padding(10).style(theme::Button::Primary);
                if !self.scanning {
                    scan_button = scan_button.on_press(ApplicationEvent::RequestedScan)
                } else {
                    cancel_button = cancel_button.on_press(ApplicationEvent::RequestedCancel)
                }

                let app_context = column![directory_list, scan_button, cancel_button]
                    .spacing(20)
                    .max_width(200);
                container(app_context)
                    .height(Length::Fill)
                    .center_y()
                    .into()
            }
        }
    }
    fn title(&self) -> String {
        String::from("Disk Analyzer")
    }
    fn update(&mut self, message: ApplicationEvent) -> Command<ApplicationEvent> {
        match message {
            ApplicationEvent::DropdownSelected => Command::none(),
            ApplicationEvent::DriveSelected(drive) => {
                self.selected_drive = Some(drive);
                Command::none()
            }
            ApplicationEvent::RequestedScan => {
                self.scanning = true;
                self.pressed_cancel = false;
                match self.selected_drive.clone() {
                    Some(drive) => {
                        let (send, recv) = channel();
                        self.cancel_sender = Some(send);

                        let selected_path: PathBuf =
                            self.paths.get(&drive).expect("Letter not found").clone();
                          let (send, recv) = channel();
                        Command::perform(
                            handlers::on_scan_start(selected_path, recv),
                            ApplicationEvent::ScanFinished,
                        )
                    }
                    None => {
                        println!("No drive selected");
                        Command::none()
                    }
                }
            }
            ApplicationEvent::RequestedCancel => {
                self.pressed_cancel = true;
                self.scanning = false;
                println!("cancel requested");
                if let Some(tracker) = &self.cancel_sender {
                    tracker.send(()).unwrap();
                }
                Command::none()
            }
            ApplicationEvent::IcedEvent(event) => {
                // does not work
                // println!("{:?}", event);
                //todo: -> https://discourse.iced.rs/t/quit-application/34
                Command::none()
            }
            ApplicationEvent::Start => Command::none(),
            // ApplicationEvent::ScanEvent(event) => {
            //     println!("{:?}", event);
            //     Command::none()
            // }
            ApplicationEvent::ScanFinished(dir) => {
                self.cancel_sender = None;
                self.scan_finished = true;
                self.dir = dir;
                self.view = View::DirectoryDisplay;
                println!("scan finished");
                println!("{:?}", &self.view);
                Command::none()
            }
        }
    }
    //     fn subscription(&self) -> Subscription<ApplicationEvent> {
    //         // handlers::connect().map(ApplicationEvent::ScanEvent)
    //         let selected_path: PathBuf = self.paths
    //         .get("C")
    //         .expect("Letter not found")
    //         .clone();
    //         handlers::some_worker().map(ApplicationEvent::ScanEvent)
    //         // subscription::events().map(ApplicationEvent::IcedEvent)
    //     }
}

pub fn run(settings: Settings<<GUI as iced::Application>::Flags>) -> Result<(), iced::Error> {
    GUI::run(settings)
}

#[derive(Default, Debug)]
pub enum View {
  #[default]
    Start,
    DirectoryDisplay,
}

#[derive(Debug, Clone)]
struct ScanError;

impl From<std::io::Error> for ScanError {
    fn from(error: std::io::Error) -> Self {
        // Convert std::io::Error to your custom error type
        ScanError
    }
}

#[derive(Debug, Clone)]
pub enum ApplicationEvent {
    DropdownSelected,
    DriveSelected(String),
    RequestedScan,
    RequestedCancel,
    Start,
    // ScanEvent(handlers::Event),
    ScanFinished(directory::Directory),
    IcedEvent(iced::Event), // couldn't use
}

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
