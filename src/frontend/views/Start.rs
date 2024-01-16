
pub struct Start {
	dir: Directory,
	scan_finished: bool,
	cancel_sender: Option<Sender<()>>,
	paths: HashMap<String, PathBuf>,
	scanning: bool,
	pressed_cancel: bool,
	selected_drive: Option<String>,
}

impl View for Start {
	fn view() -> Self {

	}
}

impl Default for Start {
	fn default() -> Self {
		Start {
			scan_finished: false,
			cancel_sender: None,
			dir: Directory::default(),
			paths: directory::get_computer_drives(),
			scanning: false,
			pressed_cancel: false,
			selected_drive: None,
		}
	}
}