

	// 	let (sender, receiver) = mpsc::channel(100);
	// 	let receiver: Arc<Mutex<mpsc::Receiver<String>>> = Arc::new(Mutex::new(receiver));
	// 	let thread_receiver = Arc::clone(&receiver);
	// 	thread::spawn(move || {
	// 		let dir = "foo"; //dir_walker::read_dir(&file_path, &recv);
	// 		sender.send(dir.to_string()).expect("Couldn't send message");
	// });
	
	// 	output
	// 	.send(ApplicationEvent::ScanEvent(Connection(sender)))
	// 	.await;
	// println!("Scanning result: {}", result);

    // let column = gtk::TreeViewColumn::new();

    // column.pack_start(&cell, true);
    // column.set_title(title);

    // if is_sortable {
    //     column.set_clickable(true);
    //     column.set_sort_indicator(true);
    //     column.set_sort_column_id(id);
    // }

    // if data_func.is_some() {
    //     gtk::TreeViewColumnExt::set_cell_data_func(&column, &cell, data_func);
    // }
    // else {
    //     column.add_attribute(&cell, "text", id);
    // }
    // tree.append_column(&column);
        // let icon_data_func = Box::new(|_, render, model, iter| {
    //     let cell = render.clone().downcast().expect("Expected renderer to be CellRenderText");
    //     let model_val = model.get_value(&iter, 0);
    //     let icon_name = model_val.get::<&str>().expect("Couldn't get icon name").expect("Couldn't get icon name");

    //     if icon_name == FOLDER_ICON || icon_name == ERROR_ICON {
    //         // cell.set_property_icon_name(Some(icon_name));
    //         Some(icon_name)
    //     }
    //     else {
    //         let icon = "foo"; //get_content_type_icon(icon_name);
    //         // cell.set_property_gicon(icon.as_ref());
    //         Some(icon.as_ref())
    //     }
    // });

    
    // let percentage_data_func = Box::new(|_, render, model, iter| {
    //     // let cell = render.clone().downcast::<gtk::CellRendererText>().expect("Expected renderer to be CellRenderText");
    //     let our_size = model.get_value(&iter, 3).get::<u64>()
    //         .expect("Couldn't get size value from tree model")
    //         .expect("Couldn't get size value from tree model") as f64;
    //     let total_size = model.get_value(&iter, 2).get::<u64>()
    //         .expect("Couldn't get size value from tree model")
    //         .expect("Couldn't get size value from tree model") as f64;

    //     let percentage = (our_size / total_size) * 100.0;
    //     let formatted = format!("{:.0}%", percentage);
    //     formatted
    //     // cell.set_property_text(Some(&formatted));
    // });

        // window: Window,
    // list_store: gtk::ListStore,
    // sort_store: gtk::TreeModelSort,
    // header_bar: gtk::HeaderBar

    
// impl AnalyzerWindow {
//     fn on_row_activated(&mut self, path: PathBuf) {
//         let current = self.model.current.upgrade().expect("Shouldn't be none");
//         let current_unlocked = current.lock().unwrap();
//         let subdirs = current_unlocked.get_subdirectories();
//         let files_start_index = subdirs.len();
//         let indices = self.sort_store.convert_path_to_child_path(&path)
//             .expect("Sorted path does not correspond to real path").get_indices();
//         if indices.len() > 0 {
//             let index = indices[0] as usize;
//             if index < files_start_index { // only want directories
//                 let new_dir = &subdirs[index];
//                 if new_dir.lock().unwrap().has_error() {
//                     let msg = format!("Could not read directory contents");
//                     // let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
//                     //                                           gtk::ButtonsType::Ok, &msg);
//                     // message_box.run();
//                     // message_box.hide();
//                 }
//                 else {
//                     self.list_store.clear();
//                     self.list_store = fill_list_store(&new_dir);
//                     self.header_bar.set_subtitle(Some(new_dir.lock().unwrap().get_path()));
//                     self.model.current = Arc::downgrade(&new_dir);
//                 }
//             }
//         }
//     }

//     fn on_up_clicked(&mut self) {
//         let current = self.model.current.upgrade().expect("Current dir shouldn't be none");
//         let parent_ptr = current.lock().unwrap().get_parent();
//         if let Some(parent) = parent_ptr.upgrade() {
//             self.list_store.clear();
//             self.list_store = fill_list_store(&parent);
//             self.header_bar.set_subtitle(Some(parent.lock().unwrap().get_path()));
//             self.model.current = Arc::downgrade(&parent);
//         }
//     }
// }


// impl Update for AnalyzerWindow {
//     type Model = AnalyzerModel;
//     type ModelParam = Arc<Mutex<directory::Directory>>;
//     type Msg = AnalyzerMsg;

//     fn model(_: &Relm<Self>, dir: Self::ModelParam) -> AnalyzerModel {
//         let current_ref = Arc::downgrade(&dir);
//         AnalyzerModel {
//             root: dir,
//             current: current_ref
//         }
//     }

//     fn update(&mut self, event: AnalyzerMsg) {
//         match event {
//             AnalyzerMsg::Quit => gtk::main_quit(),
//             AnalyzerMsg::RowActivated(path) => self.on_row_activated(path),
//             AnalyzerMsg::Up => self.on_up_clicked()
//         }
//     }
// }

// impl Widget for AnalyzerWindow {
//     // type Root = Window;

//     fn root(&self) -> Self::Root {
//         self.window.clone()
//     }

//     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
//         let file_list = gtk::TreeView::new();
//         create_analyzer_columns(&file_list);

//         let file_model = gtk::ListStore::new(&[String::static_type(), String::static_type(), u64::static_type(), u64::static_type()]);
//         let sortable_store = gtk::TreeModelSort::new(&file_model);
//         sortable_store.set_sort_column_id(gtk::SortColumn::Index(3), gtk::SortType::Descending);
//         file_list.set_model(Some(&sortable_store));
//         fill_list_store(&file_model, &model.root);

//         let viewport = gtk::Viewport::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
//         viewport.add(&file_list);
        
//         let scrolled = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
//         scrolled.add(&viewport);
//         scrolled.set_vexpand(true);

//         let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//         vbox.add(&scrolled);

//         let header_bar = gtk::HeaderBar::new();
//         let up_button = gtk::Button::from_icon_name(Some("go-up"), gtk::IconSize::Menu);
//         up_button.set_tooltip_text(Some("Up"));
//         header_bar.set_title(Some("Disk Analyzer"));
//         header_bar.set_subtitle(Some(model.root.lock().unwrap().get_path()));
//         header_bar.set_show_close_button(true);
//         header_bar.pack_start(&up_button);
        
//         let window = gtk::Window::new(WindowType::Toplevel);
//         window.add(&vbox);
//         window.set_position(gtk::WindowPosition::Center);
//         window.resize(800, 600);
//         window.set_titlebar(Some(&header_bar));

//         connect!(relm, window, connect_delete_event(_, _), return (Some(AnalyzerMsg::Quit), Inhibit(false)));
//         connect!(relm, up_button, connect_clicked(_), AnalyzerMsg::Up);
//         connect!(relm, file_list, connect_row_activated(_, path, _), AnalyzerMsg::RowActivated(path.clone()));

//         AnalyzerWindow {
//             model,
//             window,
//             list_store: file_model,
//             sort_store: sortable_store,
//             header_bar: header_bar
//         }
//     }
// }


// fn on_scan_complete(&mut self, dir: Arc<Mutex<directory::Directory>>) {
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
// 					directory::ReadError::IOError(_) => {
// 							let msg = "Could not read directory contents";
// 							let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
// 																												gtk::ButtonsType::Ok, &msg);
// 							message_box.run();
// 							message_box.hide();
// 							self.reset_ui();
// 					},
// 					directory::ReadError::OperationCancelled => self.reset_ui()
// 			}
// 	}
// }
// fn on_scan_cancel() {

// }

        // Normally this channel should be empty (which is an error, but one we expect)
        // However if we try to receive and there is no error, that means the user cancelled the scan.
        // if !cancel_checker.try_recv().is_err() {
        //     return Err(ReadError::OperationCancelled);
        // }

// pub struct ConfigModel {
//     path: Option<std::path::PathBuf>,
//     relm: Relm<ConfigWindow>
// }
// #[derive(Msg)]
// pub enum ConfigMsg {
//     Quit,
//     GotPath(Option<std::path::PathBuf>),
//     StartScan,
//     GotResults(Arc<Mutex<dir_walker::Directory>>),
//     CancelScan
// }
//  impl Widget for ConfigWindow {
//     type Root = Window;
//     fn root(&self) -> Self::Root {
//         self.window.clone()
//     }
//     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
//         let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//         let file_chooser = gtk::FileChooserButton::new("Choose directory", gtk::FileChooserAction::SelectFolder);
//         let scan_button = gtk::Button::new();
//         scan_button.set_label("Scan");
//         let cancel_button = gtk::Button::new();
//         cancel_button.set_label("Cancel");
//         cancel_button.set_sensitive(false);
//         vbox.add(&file_chooser);
//         vbox.add(&scan_button);
//         vbox.add(&cancel_button);
//         vbox.set_spacing(10);
//         let window = gtk::Window::new(WindowType::Toplevel);
//         window.set_title("Choose a directory to scan");
//         window.add(&vbox);
//         window.set_position(gtk::WindowPosition::Center);
//         window.resize(300, 75);
//         window.show_all();
//         connect!(relm, scan_button, connect_clicked(_), ConfigMsg::StartScan);
//         connect!(relm, cancel_button, connect_clicked(_), ConfigMsg::CancelScan);
//         connect!(relm, file_chooser, connect_file_set(btn), ConfigMsg::GotPath(btn.get_filename()));
//         connect!(relm, window, connect_delete_event(_, _), return (Some(ConfigMsg::Quit), Inhibit(false)));
//         ConfigWindow {
//             model,
//             window,
//             file_chooser,
//             scan_button,
//             analyzer_win: None,
//             cancel_sender: None,
//             cancel_button
//         }
//     }
// }

// impl Update for ConfigWindow {
//     type Model = ConfigModel;
//     type ModelParam = ();
//     type Msg = ConfigMsg;
//     fn model(relm: &Relm<Self>, _: ()) -> ConfigModel {
//         ConfigModel {
//             path: None,
//             relm: relm.clone()
//         }
//     }
//     fn update(&mut self, event: ConfigMsg) {
//         match event {
//             ConfigMsg::Quit => gtk::main_quit(),
//             ConfigMsg::GotPath(path) => self.model.path = path,
//             ConfigMsg::StartScan => self.on_scan_start(),
//             ConfigMsg::GotResults(result) => self.on_scan_complete(result),
//             ConfigMsg::CancelScan => self.on_scan_cancel()
//         }
//     }
// }