extern crate maeld_rust;

fn main() {
    if let Err(error) = maeld_rust::solve() {
        eprintln!("Application error: {}", error);
    }
}
