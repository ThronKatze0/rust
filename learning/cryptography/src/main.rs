use std::error::Error;

use clap::{Parser, Subcommand};
use cryptography::{decrypt_file, encrypt_file, generate_key};

mod cryptography;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Encrypt(CryptoArgs),
    Decrypt(CryptoArgs),
}

#[derive(Parser, Debug)]
struct CryptoArgs {
    /// file to do things with
    filename: String,
    /// cryptography key
    key: String,
    /// path to store the generated file
    output_path: String,
}

fn main() {
    let args = Cli::parse();
     match args.command {
         Command::Encrypt(crypto_args) => {
             if let Ok(_) = encrypt_file(crypto_args.filename, )
         }
     }
}

fn parse_key(arg_key: String) -> Result<[u8; 32], &'static str> {
    let bytes = arg_key.as_bytes();
    if bytes.len() != 32 {
        return Err("Invalid key length");
    }
    let mut array = [0_u8; 32];
    array.iter().enumerate().for_each(|(i, &byte)| array[i] = byte);
    Ok(array)
}
