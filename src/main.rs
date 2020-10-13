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

    input_path::show(builder.clone(), move |directory| {
       analyzer::show(builder.clone(), directory); 
    });
    
    gtk::main();
}
