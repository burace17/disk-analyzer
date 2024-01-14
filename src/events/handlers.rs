use std::{thread, sync::{Arc, Mutex}};
use iced_futures::subscription;	
use iced_futures::futures::channel::mpsc;
pub enum Event {
	Ready(mpsc::Sender<Input>),
	WorkFinished,
	// ...
}
enum Input {
	DoSomeWork,
}
enum State {
	Starting,
	Ready(mpsc::Receiver<Input>),
}
pub fn on_scan_start(path: Option<std::path::PathBuf>) {
	struct ScanningWorker; 
	if let Some(file_path) = path.clone() {
			// let stream = self.model.relm.stream().clone();
			// let (_, sender) = mpsc::channel();
			subscription::channel(std::any::TypeId::of::<ScanningWorker>(), 100, |mut output| async move {
        let mut state = State::Starting;

        loop {
            match &mut state {
                State::Starting => {
                    // Create channel
                    let (sender, receiver) = mpsc::channel(100);

                    // Send the sender back to the application
                    output.send(Event::Ready(sender)).await;

                    // We are ready to receive messages
                    state = State::Ready(receiver);
                }
                State::Ready(receiver) => {
                    use iced_futures::futures::StreamExt;

                    // Read next input sent from `Application`
                    let input = receiver.select_next_some().await;

                    match input {
                        Input::DoSomeWork => {
                            // Do some async work...

                            // Finally, we can optionally produce a message to tell the
                            // `Application` the work is done
                            output.send(Event::WorkFinished).await;
                        }
                    }
                }
            }
        }
    });

			// let (_, sender) = x::new(move |dir| {
			// 		stream.emit(ConfigMsg::GotResults(dir));
			// });
			// let (send, recv) = channel();

			// thread::spawn(move || {
			// 		let dir = "foo"; //dir_walker::read_dir(&file_path, &recv);
			// 		sender.send(dir).expect("Couldn't send message");
			// });
	}
}
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
