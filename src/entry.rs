/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![windows_subsystem = "windows"]
#![feature(start)]

use iced::{widget::{button, column, text, Column}, Settings, Command, Application, Theme};
use iced::executor;
mod dir_walker;
mod analyzer;
mod config_window;

struct Counter {
   value: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
   IncrementPressed,
   DecrementPressed
}

impl Application for Counter {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

   // __x: () = unused variable with unspecified type
   // in contrast to
   // y: int
   fn new(__flags: ()) -> (Counter, Command<Self::Message>) {
      (Counter { value: 0 }, Command::none())
      // Counter { value: 0 }
   }
   fn view(&self) -> Column<Message> {
      column![
         button("+").on_press(Message::IncrementPressed),
         text(self.value).size(50),
         button("-").on_press(Message::DecrementPressed)
      ]
   }
   fn title(&self) -> String {
      String::from("A cool application")
  }
   fn update(&mut self, message: Message) {
      match message {
         Message::IncrementPressed => { self.value += 1 },
         Message::DecrementPressed => { self.value -= 1 },
      }
   }
}

#[start]
fn init() -> iced::Result {
   Counter::run(Settings::default());
   // config_window::ConfigWindow::run(()).unwrap(); 
}
