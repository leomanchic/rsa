use std::fs::File;

use clap::Parser;
use num_bigint::BigInt;
// use num_bigint::BigInt;
use rsamixed::{blowfish::blowcrypt, rsa};

#[derive(Parser)]
struct Cli {
    /// Bit length for RSA
    #[arg(short, long, value_name = "len_of_key")]
    bit_len: u32,
    // path: std::path::PathBuf,
    /// Key for Blowfish
    #[arg(short, long, value_name = "key")]
    key: String,
}

fn main() {
    let args = Cli::parse();
    //Private key for Blowfish Person1
    let key = args.key;
    //Generation keys for blowfish Person1
    let bf = blowcrypt::Blowfish::new(key.as_bytes()).unwrap();

    let mut txt = b"Leomanchic".to_vec();
    println!("Без шифрования от Person1:{:02x?}", txt);
    //Blowfish encryption Person1
    bf.encrypt_block((&mut txt[..8]).try_into().unwrap());

    println!("Зашифровано с помощью blowfish Person1\n{:02x?}", txt);

    let (e, d, n) = rsamixed::rsa::rsacrypt::processing(args.bit_len).unwrap();

    //Person 2
    let key_2: String = "hollysh".to_string();
    let bf_1 = blowcrypt::Blowfish::new(key_2.as_bytes()).unwrap();

    let mut txt_2 = b"Maksimka".to_vec();
    println!("Без шифрования (Person2):{:02x?}", txt_2);
    //Blowfish encryption Person2
    bf_1.encrypt_block((&mut txt_2[..8]).try_into().unwrap());

    println!("Зашифровано с помощью blowfish{:02x?}", txt_2);

    let encrypt = rsamixed::rsa::rsacrypt::encryptinon(&e, &n, key_2.as_bytes()).unwrap();

    //Person 1
    let decr = rsamixed::rsa::rsacrypt::decryption(encrypt, d, n).unwrap();
    let decrypted_xor_key: Vec<u8> = decr.into_iter().map(|x| x.to_bytes_be().1[0]).collect();

    let bf_2 = blowcrypt::Blowfish::new(&decrypted_xor_key).unwrap();

    bf_2.decrypt_block((&mut txt_2[..8]).try_into().unwrap());
    println!("Расшифровка сообщения(от Person2){:02x?}", txt_2);

    // let (e, d, n) = rsamixed::rsa::rsacrypt::processing(args.bit_len).unwrap();
    // println!("{}",e);
    // let temp_e = e.to_bytes_be();
    // let file = File::create("file_e.txt");

    // let new_B = BigInt::from_bytes_be(temp_e.0,&temp_e.1);

    // println!("{:?}\n {}",temp_e,new_B);

    // rsamixed::config::configen::toml_gen(format!("{}", e), format!("{}", d), format!("{}", key));

    // //Encryption Blowfish key with rsa public key
    // let enc = rsamixed::rsa::rsacrypt::encryptinon(&e, &n, key.as_bytes()).unwrap();

    // let seria = rsamixed::rsa::rsacrypt::serialization(enc).unwrap();
    // let desir = rsamixed::rsa::rsacrypt::deseriallization(seria).unwrap();

    // //Decryption of Blowfish key with RSA private key
    // let decr = rsamixed::rsa::rsacrypt::decryption(desir, d, n).unwrap();

    // let decrypted_xor_key: Vec<u8> = decr.into_iter().map(|x| x.to_bytes_be().1[0]).collect();
    // // let text_decryption = rsamixed::rsa::rsacrypt::xor(&decrypted_xor_key, &text);

    // //Decrypion of messege with decrypted blowfish key
    // let bf = blowcrypt::Blowfish::new(&decrypted_xor_key).unwrap();

    // bf.decrypt_block((&mut txt[..8]).try_into().unwrap());
    // println!("Расшифровка сообщения {:02x?}", txt);

    // println!("{:?}", text_decryption)
}
