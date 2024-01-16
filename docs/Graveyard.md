

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

