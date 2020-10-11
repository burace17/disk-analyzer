mod dir_walker;
mod analyzer;
mod input_path;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let builder_clone = builder.clone();
    input_path::show(&builder, move |directory| {
       analyzer::show(&builder_clone, directory); 
    });
    
    gtk::main();
}
