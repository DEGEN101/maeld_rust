extern crate meald_rust;

fn main() {
    if let Err(error) = meald_rust::solve() {
        eprintln!("Application error: {}", error);
    }
}
