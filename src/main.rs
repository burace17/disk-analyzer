/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![windows_subsystem = "windows"]
mod dir_walker;
mod analyzer;
mod config_window;
use relm::Widget;

fn main() {
   config_window::ConfigWindow::run(()).unwrap(); 
}
