use clap::Parser;

#[derive(Parser)]
struct Args {
    /// the string to be printed
    message: String,
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    println!("{}", args.message);
}
