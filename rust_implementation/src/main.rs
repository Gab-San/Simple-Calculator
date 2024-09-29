use std::process;

fn main() {

    // Interested only in the error
    if let Err(e) = calculator::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}