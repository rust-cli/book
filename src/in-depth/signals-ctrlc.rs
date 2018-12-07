fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    }).expect("Error setting Ctrl-C handler");

    // ...
}
