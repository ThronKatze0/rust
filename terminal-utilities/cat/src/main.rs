use clap::Parser;
use std::fs::read_to_string;

/// Simple program ls
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path
    path: String,
}

fn main() {
    let args = Args::parse();
    match read_to_string(args.path) {
        Ok(content) => println!("{}", content),
        Err(_) => println!("File not found"),
    }
}
