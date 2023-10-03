use clap::Parser;
use rsamixed::blowfish::blowcrypt;
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

fn main() {
    let key ="verysecretpasswd".as_bytes();
    let bf = blowcrypt::Blowfish::new(key).unwrap();

    let mut txt = b"gdhdfjfg".to_vec();
    println!("{:02x?}", txt);

    bf.encrypt_block((&mut txt[..8]).try_into().unwrap());

    println!("{:02x?}", txt);
   
   

    let args = Cli::parse();
    let text = args.messege.as_bytes();
    println!("{:?}", text);

    // let key_xor = "leonid".as_bytes();
    // let text = rsamixed::rsa::rsacrypt::xor(key_xor, text);

    println!("{:?}", text);

    let (e, d, n) = rsamixed::rsa::rsacrypt::processing(args.bit_len).unwrap();

    let enc = rsamixed::rsa::rsacrypt::encryptinon(&e, &n, key).unwrap();
    // println!("{:?}", enc);

    let seria = rsamixed::rsa::rsacrypt::serialization(enc).unwrap();
    let desir = rsamixed::rsa::rsacrypt::deseriallization(seria).unwrap();
    let decr = rsamixed::rsa::rsacrypt::decryption(desir, d, n).unwrap();

    let decrypted_xor_key: Vec<u8> = decr.into_iter().map(|x| x.to_bytes_be().1[0]).collect();
    // let text_decryption = rsamixed::rsa::rsacrypt::xor(&decrypted_xor_key, &text);
    let bf = blowcrypt::Blowfish::new(&decrypted_xor_key).unwrap();

    bf.decrypt_block((&mut txt[..8]).try_into().unwrap());
    println!("{:02x?}", txt);

    // println!("{:?}", text_decryption)
}
