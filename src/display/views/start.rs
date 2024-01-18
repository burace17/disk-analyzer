
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

fn display_starting_view(app: &mut GUI) -> Element<ApplicationEvent> {
	let drives_as_strings: Vec<String> = self.paths.keys().cloned().collect();
	let directory_list = pick_list(
			drives_as_strings,
			self.selected_drive.clone(),
			ApplicationEvent::DriveSelected,
	)
	.placeholder("Select a directory...");
	let mut scan_button = button("scan").padding(10).style(theme::Button::Primary);
	let mut cancel_button = button("cancel").padding(10).style(theme::Button::Primary);
	if !self.scanning {
			scan_button = scan_button.on_press(ApplicationEvent::RequestedScan)
	} else {
			cancel_button = cancel_button.on_press(ApplicationEvent::RequestedCancel)
	}

	let app_context = column![directory_list, scan_button, cancel_button]
			.spacing(20)
			.max_width(200);
	container(app_context)
			.height(Length::Fill)
			.center_y()
			.into()
}