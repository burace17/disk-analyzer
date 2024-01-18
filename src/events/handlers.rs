use crate::{
    application::ApplicationEvent,
    directory::{self, Directory},
};
use async_tungstenite::tungstenite;
use futures::{channel::mpsc::Sender, future, stream::FuturesUnordered};
use futures_util;
use iced::Command;
use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex, Weak,
    },
    thread,
};

pub async fn on_scan_start(file_path: PathBuf, recv: Receiver<()>) -> Directory {
    let dir = directory::read_dir(&file_path, &recv);
    dir
}
