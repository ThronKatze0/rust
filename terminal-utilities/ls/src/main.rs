use clap::Parser;
use core::panic;
use std::env::current_dir;
use std::fs::Metadata;
use std::fs::{self};
use std::os::unix::fs::MetadataExt;

#[derive(Parser, Debug)]
struct Args {
    /// if it should be displayed as a list
    #[arg(short, long)]
    list: bool,
}

fn main() {
    let args = Args::parse();
    let current_dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!(),
    };
    match fs::read_dir(&current_dir) {
        Ok(entries) => entries.for_each(|entry| match entry {
            Ok(entry) => {
                let filename: String = entry
                    .file_name()
                    .into_string()
                    .expect("filename is not readable");
                let metadata: Metadata =
                    entry.metadata().expect("Can't read metadata of this file");
                match args.list {
                    false => print!("{} ", filename),
                    true => {
                        let mode: u32 = metadata.mode();
                        let filetype: String = if metadata.is_dir() {
                            'd'.to_string()
                        } else {
                            '-'.to_string()
                        };
                        let user_permissions: String = format!(
                            "{}{}{}",
                            if mode & 0o400 != 0 { 'r' } else { '-' },
                            if mode & 0o200 != 0 { 'w' } else { '-' },
                            if mode & 0o100 != 0 { 'x' } else { '-' },
                        );
                        let group_permissions: String = format!(
                            "{}{}{}",
                            if mode & 0o040 != 0 { 'r' } else { '-' },
                            if mode & 0o020 != 0 { 'w' } else { '-' },
                            if mode & 0o010 != 0 { 'x' } else { '-' },
                        );
                        let other_permissions: String = format!(
                            "{}{}{}",
                            if mode & 0o004 != 0 { 'r' } else { '-' },
                            if mode & 0o002 != 0 { 'w' } else { '-' },
                            if mode & 0o001 != 0 { 'x' } else { '-' },
                        );
                        let owner_user: u32 = metadata.uid();
                        let owner_group: u32 = metadata.gid();
                        let size: u64 = metadata.len();
                        // I can't figure out how to format this
                        let _created = metadata.created().expect("Get a unix system, normie");
                        println!(
                            "{}",
                            format!(
                                "{}{} {} {} {} {} {} {}",
                                filetype,
                                user_permissions,
                                group_permissions,
                                other_permissions,
                                owner_user,
                                owner_group,
                                size,
                                filename
                            )
                        )
                    }
                }
            }
            Err(_) => eprintln!("Cannot read this file"),
        }),
        Err(_) => eprintln!("Some Error occured lol"),
    }
}
