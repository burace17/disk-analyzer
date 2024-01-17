use std::{thread, sync::{Arc, Mutex, Weak, mpsc}, path::PathBuf};
use futures::{channel::mpsc::Sender, future, stream::FuturesUnordered};
use iced::Command;
use crate::{application::ApplicationEvent, directory::{self, Directory}};
use async_tungstenite::tungstenite;
use futures_util;

// use iced_futures::{Subscription, subscription, futures::{stream::Scan, sink::SinkExt, self, channel::mpsc::{self, Receiver}}};


#[derive(Debug, Clone)]
pub enum Event {
	Ready(mpsc::Sender<Input>),
	WorkFinished,
	// ...
}

pub enum Input {
	DoSomeWork,
	// ...
}

enum State {
	Starting,
	Ready(mpsc::Receiver<Input>),
}

pub async fn on_scan_start(file_path: PathBuf) -> Directory {
	let dir = Directory::new("C", Weak::new(), &"path");
	// directory::read_dir(&file_path, &recv);
	dir
	
	
}

// fn on_scan_complete(&mut self, dir: Arc<Mutex<directory::Directory>>) {
// 	self.cancel_sender = None;
// 	let dir_clone = dir.clone();
// 	let error = dir.lock().unwrap().get_error().clone();
// 	match error {
// 			None => {
// 					self.window.hide();
// 					let analyzer_win = init::<analyzer::AnalyzerWindow>(dir_clone).expect("Couldn't init");
// 					analyzer_win.widget().show_all();
// 					self.analyzer_win = Some(analyzer_win);
// 			},
// 			Some(e) => match e {
// 					directory::ReadError::IOError(_) => {
// 							let msg = "Could not read directory contents";
// 							let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
// 																												gtk::ButtonsType::Ok, &msg);
// 							message_box.run();
// 							message_box.hide();
// 							self.reset_ui();
// 					},
// 					directory::ReadError::OperationCancelled => self.reset_ui()
// 			}
// 	}
// }
// fn on_scan_cancel() {

// }