use clap::Parser;
use rsa;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long, value_name = "len_of_key")]
    bit_len: u32,
    // path: std::path::PathBuf,
}

use std::str;
fn main() {
    let args = Cli::parse();
    let text = "Hello".as_bytes();
    println!("{:?}", text);
    // let bit_len = 1024;

    let (e, d, n) = rsa::processing(args.bit_len).unwrap();

    let enc = rsa::encryptinon(&e, &n, text).unwrap();
    println!("{:?}", enc);

    let seria = rsa::serialization(enc).unwrap();
    let desir = rsa::deseriallization(seria).unwrap();
    let decr = rsa::decryption(desir, d, n).unwrap();
    for i in decr {
        print!("{}", str::from_utf8(&i.to_bytes_be().1).unwrap())
    }
}
