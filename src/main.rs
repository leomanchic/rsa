use clap::Parser;
// use num_bigint::BigInt;
use rsamixed::blowfish::blowcrypt;

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
    //Private key for Blowfish
    let key = args.key;
    //Generation keys for blowfish
    let bf = blowcrypt::Blowfish::new(key.as_bytes()).unwrap();

    let mut txt = b"Leomanchic".to_vec();
    println!("Без шифрования:{:02x?}", txt);
    //Blowfish encryption
    bf.encrypt_block((&mut txt[..8]).try_into().unwrap());

    println!("Зашифровано с помощью blowfish{:02x?}", txt);

    

    let (e, d, n) = rsamixed::rsa::rsacrypt::processing(args.bit_len).unwrap();

    rsamixed::config::configen::toml_gen(format!("{}", e), format!("{}", d), format!("{}", key));

    //Encryption Blowfish key with rsa public key
    let enc = rsamixed::rsa::rsacrypt::encryptinon(&e, &n, key.as_bytes()).unwrap();

    let seria = rsamixed::rsa::rsacrypt::serialization(enc).unwrap();
    let desir = rsamixed::rsa::rsacrypt::deseriallization(seria).unwrap();

    //Decryption of Blowfish key with RSA private key
    let decr = rsamixed::rsa::rsacrypt::decryption(desir, d, n).unwrap();

    let decrypted_xor_key: Vec<u8> = decr.into_iter().map(|x| x.to_bytes_be().1[0]).collect();
    // let text_decryption = rsamixed::rsa::rsacrypt::xor(&decrypted_xor_key, &text);

    //Decrypion of messege with decrypted blowfish key
    let bf = blowcrypt::Blowfish::new(&decrypted_xor_key).unwrap();

    bf.decrypt_block((&mut txt[..8]).try_into().unwrap());
    println!("Расшифровка сообщения {:02x?}", txt);

    // println!("{:?}", text_decryption)
}
