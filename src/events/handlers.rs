use std::{thread, sync::{Arc, Mutex}, path::PathBuf};
use futures::{channel::mpsc::Sender, future, stream::FuturesUnordered};
use iced_futures::{Subscription, subscription, futures::{stream::Scan, sink::SinkExt, self, channel::mpsc::{self, Receiver}}};
use crate::application::ApplicationEvent;
use async_tungstenite::tungstenite;
use futures_util;

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

pub(crate) fn some_worker() -> Subscription<Event> {
	struct SomeWorker;

	subscription::channel(std::any::TypeId::of::<SomeWorker>(), 100, |mut output| async move {
			let mut state = State::Starting;

			loop {
					match &mut state {
							State::Starting => {
									let (sender, receiver) = mpsc::channel(100);
									output.send(Event::Ready(sender)).await;
									state = State::Ready(receiver);
							}
							State::Ready(receiver) => {
								use iced_futures::futures::StreamExt;
								futures_util::select! {
										received = receiver.select_next_some() => {
												match received {
													Input::DoSomeWork => {
														output.send(Event::WorkFinished).await;
														state = State::Starting
													}
											}
										}
										complete => continue,
									}
							}
					}
			}
	})
}

