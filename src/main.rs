mod dir_walker;
mod analyzer;
mod input_path;
use relm::Widget;

fn main() {
   input_path::ConfigWindow::run(()).unwrap(); 
}
