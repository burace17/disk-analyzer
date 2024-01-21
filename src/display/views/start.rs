use iced::widget::{button, container, pick_list};
use iced::{theme, Length};
use iced::{widget::column, Element};
use std::{collections::HashMap, path::PathBuf, sync::mpsc::Sender};

use crate::application::{ApplicationEvent, View, GUI};
use crate::logic::directory::directory::{self, get_computer_drives, Directory};

pub struct Start {
    dir: Directory,
    scan_finished: bool,
    cancel_sender: Option<Sender<()>>,
    paths: HashMap<String, PathBuf>,
    scanning: bool,
    pressed_cancel: bool,
    selected_drive: Option<String>,
}

impl Default for Start {
    fn default() -> Self {
        Start {
            scan_finished: false,
            cancel_sender: None,
            dir: Directory::default(),
            paths: get_computer_drives(),
            scanning: false,
            pressed_cancel: false,
            selected_drive: None,
        }
    }
}

pub fn display_starting_view(app: &GUI) -> Element<ApplicationEvent> {
    let drives_as_strings: Vec<String> = app.paths.keys().cloned().collect();
    let directory_list = pick_list(
        drives_as_strings,
        app.selected_drive.clone(),
        ApplicationEvent::DriveSelected,
    )
    .placeholder("Select a directory...");
    let mut scan_button = button("scan").padding(10).style(theme::Button::Primary);
    let mut cancel_button = button("cancel").padding(10).style(theme::Button::Primary);
    if !app.scanning {
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
