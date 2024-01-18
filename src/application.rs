/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(unused_imports)]
use std::collections::HashMap;
use std::io::Error;
use std::iter::Scan;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

use crate::logic::directory::get_computer_drives;
use crate::logic::{
  directory::Directory, 
  analyzer::ViewColumn
};
use crate::display::views::{directory, start};
use crate::events::handlers::{self, on_scan_request};

use iced::widget::button::StyleSheet;
use iced::widget::{button, column, container, pick_list, row, text, Column, Container, Row, Text};
use iced::{executor, subscription, window, Renderer};
use iced::{theme, Application, Command, Element, Event, Length, Settings, Subscription, Theme};

#[derive(Default)]
pub struct GUI {
    view: View,
    pub dir: Directory,
    scan_finished: bool,
    pub cancel_sender: Option<Sender<()>>,
    pub paths: HashMap<String, PathBuf>,
    pub scanning: bool,
    pub pressed_cancel: bool,
    pub selected_drive: Option<String>,
}
// use super::events::handlers::on_scan_start;

/* top level app presentation interface */
impl Application for GUI {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ApplicationEvent;
    type Theme = Theme;
    fn new(__flags: ()) -> (GUI, Command<ApplicationEvent>) {
        (
            GUI {
                view: View::Start,
                scan_finished: false,
                cancel_sender: None,
                dir: Directory::default(),
                paths: get_computer_drives(),
                scanning: false,
                pressed_cancel: false,
                selected_drive: None,
            },
            Command::none(),
        )
    }
    fn view(&self) -> Element<ApplicationEvent> {
        match self.view {
            View::DirectoryDisplay => directory::directory_display_view(self),
            View::Start => start::display_starting_view(self)
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
            ApplicationEvent::RequestedScan => on_scan_request(self),
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
    ScanFinished(Directory),
    IcedEvent(iced::Event), // couldn't use
}
