#![windows_subsystem = "windows"]
mod dir_walker;
mod analyzer;
mod config_window;
use relm::Widget;

fn main() {
   config_window::ConfigWindow::run(()).unwrap(); 
}
