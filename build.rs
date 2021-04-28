use std::process::{exit, Command};

fn main() {
    Command::new("cargo")
        .args(&["fmt"])
        .status()
        .expect("Failed to run `cargo fmt`.");
}
