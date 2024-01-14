// use std::{thread, sync::{Arc, Mutex}};

// fn on_scan_start(&mut self) {
// 	if let Some(file_path) = self.model.path.clone() {
// 			let stream = self.model.relm.stream().clone();
// 			let (_, sender) = Channel::new(move |dir| {
// 					stream.emit(ConfigMsg::GotResults(dir));
// 			});
// 			let (send, recv) = channel();
// 			self.cancel_sender = Some(send);
// 			self.scan_button.set_label("Reading...");
// 			self.scan_button.set_sensitive(false);
// 			self.file_chooser.set_sensitive(false);
// 			self.cancel_button.set_sensitive(true);
// 			thread::spawn(move || {
// 					let dir = dir_walker::read_dir(&file_path, &recv);
// 					sender.send(dir).expect("Couldn't send message");
// 			});
// 	}
// }
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
