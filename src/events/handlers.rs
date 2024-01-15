use std::{thread, sync::{Arc, Mutex}, path::PathBuf};
use iced_futures::{Subscription, subscription, futures::{stream::Scan, sink::SinkExt, self, channel::mpsc::{self, Receiver}}};
use crate::application::ApplicationEvent;
use async_tungstenite::tungstenite;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    // Start(PathBuf),
    Cancelled,
    Completed,
}

pub enum State {
	Left, 
	Right(Receiver<ScanEvent>)
}


#[derive(Debug, Clone)]
pub enum Event {
    Ready(mpsc::Sender<ScanEvent>),
    WorkFinished,
}

pub fn connect() -> Subscription<Event> {
	struct Connect;

	subscription::channel(
			std::any::TypeId::of::<Connect>(),
			100,
			|mut output| async move {
					let mut state = State::Left;

					loop {
						match &mut state {
								State::Left => {
										let (sender, receiver) = mpsc::channel(100);
										output.send(Event::Ready(sender)).await;
										state = State::Right(receiver);
								}
								State::Right(receiver) => {
										use futures::stream::StreamExt;

										let input = receiver.select_next_some().await;
										match input {
												ScanEvent::Completed => {
														output.send(Event::WorkFinished).await;
												}
												ScanEvent::Cancelled => {
													output.send(Event::WorkFinished).await;
												}
										}
								}
						}
				}
			}
	)
}

pub fn on_scan_start(file_path: std::path::PathBuf) -> Subscription<ScanEvent> {
	// let (_, sender) = x::new(move |dir| {
	// 		stream.emit(ConfigMsg::GotResults(dir));
	// });
	// let (send, recv) = channel();
	// self.cancel_sender = Some(send);

	struct Scanner;


	// todo: fix didn't exit correctly
	subscription::channel(std::any::TypeId::of::<Scanner>(), 100, 
	|mut output| async move {
			let mut state = ScanEvent::Completed;
			println!("blep");
			let mut state = ScanEvent::Cancelled;

			loop {
					match &mut state {
							ScanEvent::Cancelled => {
									const ECHO_SERVER: &str = "ws://127.0.0.1:3030";

							}
							ScanEvent::Completed => {
									// let mut fused_websocket = websocket.by_ref().fuse();
							}
					}
			}
		}
	)
}


	// 	let (sender, receiver) = mpsc::channel(100);
	// 	let receiver: Arc<Mutex<mpsc::Receiver<String>>> = Arc::new(Mutex::new(receiver));
	// 	let thread_receiver = Arc::clone(&receiver);
	// 	thread::spawn(move || {
	// 		let dir = "foo"; //dir_walker::read_dir(&file_path, &recv);
	// 		sender.send(dir.to_string()).expect("Couldn't send message");
	// });
	
	// 	output
	// 	.send(ApplicationEvent::ScanEvent(Connection(sender)))
	// 	.await;
	// println!("Scanning result: {}", result);

// fn on_scan_complete(&mut self, dir: Arc<Mutex<dir_walker::Directory>>) {
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
// 					dir_walker::ReadError::IOError(_) => {
// 							let msg = "Could not read directory contents";
// 							let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
// 																												gtk::ButtonsType::Ok, &msg);
// 							message_box.run();
// 							message_box.hide();
// 							self.reset_ui();
// 					},
// 					dir_walker::ReadError::OperationCancelled => self.reset_ui()
// 			}
// 	}
// }
// fn on_scan_cancel(&self) {
// 	self.cancel_button.set_sensitive(false);
// 	if let Some(tracker) = &self.cancel_sender {
// 			tracker.send(()).unwrap();
// 	}
// }
