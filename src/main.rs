use clap::Parser;
use num_bigint::BigInt;
use pem::{encode, Pem};
// use sha256::{digest, try_digest};
use std::{
    fs::File,
    io::{Read, Write},
};
// use num_bigint::BigInt;
use rsamixed::{blowfish::blowcrypt, rsa};

#[derive(Parser)]
struct Cli {
    /// Bit length for RSA
    #[arg(short, long, value_name = "len_of_key")]
    bit_len: u32,
    /// Key for Blowfish
    #[arg(short, long, value_name = "key")]
    key: String,
    /// File to write key
    #[arg(short, long, value_name = "file")]
    file: String,
}

fn main() {
    let args = Cli::parse();
    //Private key for Blowfish Person1
    let key = args.key;
    //Generation keys for blowfish Person1
    let bf = blowcrypt::Blowfish::new(key.as_bytes()).unwrap();

    let mut txt = b"Leomanchhicghicg".to_vec();
    println!("Без шифрования от Person1:{:02x?}", txt);
    //Blowfish encryption Person1
    bf.encrypt_block((&mut txt[..8]).try_into().unwrap());
    // bf.encrypt_block((&mut txt[8..16]).try_into().unwrap());

    println!("Зашифровано с помощью blowfish Person1\n{:02x?}", txt);

    let (e, d, n) = rsamixed::rsa::rsacrypt::processing(args.bit_len).unwrap();

    let mut file = File::create(args.file).unwrap();
    let (serialized_e,serialized_n,serialized_d) = (e.to_bytes_be().1,n.to_bytes_be().1,d.to_bytes_be().1);

    rsamixed::config::configen::pem_gen(&serialized_e, &serialized_n, &serialized_d);
    
    // //Person 2
    // let key_2: String = "hollysh".to_string();
    // let bf_1 = blowcrypt::Blowfish::new(key_2.as_bytes()).unwrap();

    // let mut txt_2 = b"Maksimka".to_vec();
    // println!("Без шифрования (Person2):{:02x?}", txt_2);
    // //Blowfish encryption Person2
    // bf_1.encrypt_block((&mut txt_2[..8]).try_into().unwrap());

    // println!("Зашифровано с помощью blowfish{:02x?}", txt_2);

    // let encrypt = rsamixed::rsa::rsacrypt::encryptinon(&e, &n, key_2.as_bytes()).unwrap();

    // //Person 1
    // let decr = rsamixed::rsa::rsacrypt::decryption(encrypt, d, n).unwrap();
    // let decrypted_xor_key: Vec<u8> = decr.into_iter().map(|x| x.to_bytes_be().1[0]).collect();

    // let bf_2 = blowcrypt::Blowfish::new(&decrypted_xor_key).unwrap();

    // bf_2.decrypt_block((&mut txt_2[..8]).try_into().unwrap());
    // println!("Расшифровка сообщения(от Person2){:02x?}", txt_2);

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

#[cfg(test)]
#[test]
fn it_works() {
    // use pem::{encode, Pem};

    // let input = String::from("hello");
    // let bit_len: u32 = 512;
    // let (e, d, n) = rsamixed::rsa::rsacrypt::processing(bit_len).unwrap();
    // let serialized_e = e.to_bytes_be().1;

    // println!("e {}", e);

    // let val = Pem::new("PUBLIC KEY", serialized_e);
    // println!("PEM {:?}", val);extern crate crypto;
    // let public = pem::encode(&val);
    // println!("PEM {:?}", public);
    // let back = pem::parse(public).unwrap();
    // let backed = BigInt::from_signed_bytes_be(back.contents());
    // // println!("{}", backed);
    // assert_eq!(e, backed)
}

#[test]
fn file_read() {
    let mut buf: Vec<u8> = Vec::new();
    let mut file = File::open("config.pem").unwrap();
    file.read_to_end(&mut buf);
    let key = pem::parse_many(buf).unwrap();
    let expo = BigInt::from_signed_bytes_be(key[0].contents());
    let n = BigInt::from_signed_bytes_be(key[1].contents());
    let d = BigInt::from_signed_bytes_be(key[2].contents());
    assert_ne!(d, expo);
    // println!("{}\n{}\n\n{}\n{}\n", key[0].tag(), expo, key[1].tag(), n);
    // println!("{}\n{}", key[2].tag(), d);

    let key = "leon";
    println!("key as b{:?}", key.as_bytes());
    //Generation keys for blowfish Person1
    let bf = blowcrypt::Blowfish::new(key.as_bytes()).unwrap();

    let mut txt = b"Leonardo".to_vec();
    println!("Без шифрования текст:{:02x?}", txt);
    //Blowfish encryption Person1
    bf.encrypt_block((&mut txt[..8]).try_into().unwrap());
    let encrypted_blow_key = rsa::rsacrypt::encryptinon(&expo, &n, key.as_bytes()).unwrap();
    let serialize_blow_key = rsa::rsacrypt::serialization(encrypted_blow_key).unwrap();

    let mut file = File::create("sesion_key.txt").unwrap();
    file.write_all(&serialize_blow_key).unwrap();

    // println!("Сообщение зашфрованное по ключу blowfish {:?}\n Зашифрованный публичным ключом, ключ blowfish {:?}", txt,encrypted_blow_key.unwrap())
}
#[test]
fn decryption() {
    let mut buf: Vec<u8> = Vec::new();
    let mut file = File::open("config.pem").unwrap();
    file.read_to_end(&mut buf);
    let key = pem::parse_many(buf).unwrap();
    let d = BigInt::from_signed_bytes_be(key[2].contents());
    let n = BigInt::from_signed_bytes_be(key[1].contents());

    let mut buf: Vec<u8> = Vec::new();
    let mut file1 = File::open("sesion_key.txt").unwrap();
    file1.read_to_end(&mut buf);
    let decripted_key = rsa::rsacrypt::deseriallization(buf).unwrap();
    let decripted_key = rsa::rsacrypt::decryption(decripted_key, d, n);
    // let serialized_key = rsa::rsacrypt::serialization(decripted_key.unwrap());
    println!("{:?}", decripted_key);
}
