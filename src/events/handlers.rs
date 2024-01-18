use crate::{
    application::{ApplicationEvent, GUI}, logic::directory::{Directory, read_dir},
};
use async_tungstenite::tungstenite;
use futures::{channel::mpsc::Sender, future, stream::FuturesUnordered};
use futures_util;
use iced::Command;
use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, channel},
        Arc, Mutex, Weak,
    },
    thread,
};

pub async fn on_scan_start(file_path: PathBuf, recv: Receiver<()>) -> Directory {
    let dir = read_dir(&file_path, &recv);
    dir
}

pub fn on_scan_request(app: &mut GUI) -> Command<ApplicationEvent> {
    {
        app.scanning = true;
        app.pressed_cancel = false;
        match app.selected_drive.clone() {
            Some(drive) => {
                let (send, recv) = channel();
                app.cancel_sender = Some(send);

                let selected_path: PathBuf =
                    // PathBuf::from(String::from(r"C:\Users\AJ\Desktop"));
                app.paths.get(&drive).expect("Letter not found").clone();

                let (send, recv) = channel();
                Command::perform(
                    on_scan_start(selected_path, recv),
                    ApplicationEvent::ScanFinished,
                )
            }
            None => {
                println!("No drive selected");
                Command::none()
            }
        }
    }
}