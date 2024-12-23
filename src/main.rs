use std::env;

use to_rightly_divide::application::Application;

/*
try: cargo run -- --reference "Genesis 1:1" (example)
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = Application::new_kjv();
    app.process_arguments(&args);
}

/*
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
*/
