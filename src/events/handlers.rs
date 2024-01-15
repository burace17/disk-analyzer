use std::{thread, sync::{Arc, Mutex}, path::PathBuf};
use futures::channel::mpsc::Sender;
use iced_futures::{Subscription, subscription, futures::{stream::Scan, sink::SinkExt, self, channel::mpsc::{self, Receiver}}};
use crate::application::ApplicationEvent;
use async_tungstenite::tungstenite;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Completed(Sender<Event>),
		Cancelled,
}
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum State {
	Left, 
	Right(Receiver<Event>)
}

pub enum Event {
    Ready,
    WorkFinished,
}

pub fn on_scan_start(file_path: std::path::PathBuf) -> Subscription<ScanEvent> {
	struct Scanner;

	// todo: fix didn't exit correctly
	subscription::channel(std::any::TypeId::of::<Scanner>(), 100, 
	|mut output| async move {
		let mut state = State::Left;
		loop {
			match &mut state {
					State::Left => {
							let (sender, receiver) = mpsc::channel(100);
							output.send(ScanEvent::Completed(sender)).await;
							state = State::Right(receiver);
					}
					State::Right(receiver) => {
							use futures::stream::StreamExt; // why do we need stream ext?
							let input = receiver.select_next_some().await;
							output.send(ScanEvent::Cancelled).await;
						}
			}
		}
	}		
	)
}

