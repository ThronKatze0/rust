use clap::Parser;
use core::panic;
use std::env::current_dir;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    list: bool,
}

fn main() {
    let _arg = Args::parse();
    let current_dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!(),
    };
    match fs::read_dir(&current_dir) {
        Ok(entries) => entries.for_each(|entry| match entry {
            Ok(entry) => {
                let path: String = entry.path().display().to_string();
                let file_name_data: Vec<&str> = path.split("/").collect();
                print!(
                    "{} ",
                    match file_name_data.last() {
                        Some(filename) => filename,
                        None => "Somehow vec empty",
                    }
                );
            }
            Err(_) => eprintln!("Cannot read this file"),
        }),
        Err(_) => eprintln!("Some Error occured lol"),
    }
}
