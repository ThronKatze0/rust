use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Parser)]
struct Args {
    pattern: String,
}

fn main() {
    // grep: 10.38s
    // mygrep: 7.00s
    // on some random benchmark
    let args = Args::parse();
    let pattern: &str = &args.pattern;
    grep_dir(Path::new("./"), pattern);
}

fn grep_dir(dir_path: &Path, pattern: &str) {
    let paths = match fs::read_dir(dir_path) {
        Ok(paths) => paths,
        Err(_) => return (),
    };
    paths.par_bridge().for_each(|entry| {
        let path = match entry {
            Ok(path) => path.path(),
            Err(_) => return (),
        };
        if path.is_dir() {
            grep_dir(path.as_path(), pattern);
        } else {
            let file = match fs::File::open(path) {
                Ok(file) => file,
                Err(_) => return (),
            };
            let buf = BufReader::new(file);
            buf.lines().par_bridge().for_each(|line_entry| {
                let line = match line_entry {
                    Ok(line) => line,
                    Err(_) => return (),
                };
                let line = line.trim().to_string();
                line.match_indices(pattern)
                    .map(|(i, _)| i)
                    .for_each(|item| {
                        println!(
                            "{} => {}{}{}",
                            if let Some(path_str) = dir_path.to_str() {
                                path_str
                            } else {
                                ""
                            },
                            if let Some(slice) =
                                line.get(if item >= 20 { item - 20 } else { 0 }..item)
                            {
                                slice
                            } else {
                                &line[..item]
                            },
                            pattern.red(),
                            if let Some(slice) =
                                line.get(item + pattern.len()..item + pattern.len() + 20)
                            {
                                slice
                            } else {
                                &line[item + pattern.len()..]
                            }
                        );
                    });
            })
        }
    })
}
