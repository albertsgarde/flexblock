use std::{fs, process::Command};

fn main() {
    let args = if let Ok(args_string) = fs::read_to_string("build_args.txt") {
        args_string
            .split(",")
            .map(|str| str.to_owned())
            .collect::<Vec<String>>()
    } else {
        Vec::new()
    };
    if !args.is_empty() {
        Command::new("cargo")
            .args(args)
            .status()
            .expect("Failed to run `cargo fmt`.");
    }
}
