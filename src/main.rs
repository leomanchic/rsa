use clap::Parser;
use rsa;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
struct Cli {
    /// Bit length of keys
    #[arg(short, long, value_name = "len_of_key")]
    bit_len: u32,
    // path: std::path::PathBuf,
    /// Value to encrypt
    #[arg(short, long, value_name = "messege_to_encrypt")]
    messege: String,
}

use std::str;
fn main() {
    let args = Cli::parse();
    let text = args.messege.as_bytes();
    println!("{:?}", text);

    let key_xor = "leonid".as_bytes();
    let text = rsa::xor(key_xor, text);

    println!("{:?}", text);

    let (e, d, n) = rsa::processing(args.bit_len).unwrap();

    let enc = rsa::encryptinon(&e, &n, key_xor).unwrap();
    // println!("{:?}", enc);

    let seria = rsa::serialization(enc).unwrap();
    let desir = rsa::deseriallization(seria).unwrap();
    let decr = rsa::decryption(desir, d, n).unwrap();

    let decrypted_xor_key: Vec<u8> = decr.into_iter().map(|x| x.to_bytes_be().1[0]).collect();
    let text_decryption = rsa::xor(&decrypted_xor_key, &text);
    println!("{:?}", text_decryption);
}
