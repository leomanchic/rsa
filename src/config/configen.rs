use pem::{encode, Pem};
use std::{error::Error, fs::File, io::Write};

// #[derive(Serialize)]
// struct Config {
//     keys: Keys,
// }

// #[derive(Serialize)]
// struct Keys {
//     blowfish: Blowfish_key,
//     rsa: RSA,
// }
// #[derive(Serialize)]
// struct Blowfish_key {
//     private_key: String,
// }
// #[derive(Serialize)]
// struct RSA {
//     public_key: String,
//     private_key: String,
// }

// #[warn(unused_must_use)]
// pub fn toml_gen(pub_key: String, pr_key: String, bl_key: String) {
//     let conf = Config {
//         keys: Keys {
//             blowfish: Blowfish_key {
//                 private_key: bl_key,
//             },
//             rsa: RSA {
//                 public_key: pub_key,
//                 private_key: pr_key,
//             },
//         },
//     };
//     let mut file = File::create("config.toml").unwrap();
//     let toml = toml::to_string(&conf).unwrap();
//     file.write_all(&toml.as_bytes());
// }

pub fn pem_gen(pub_expo: &[u8], module: &[u8], priv_d: &[u8], file: Option<&str>) {
    let serialized_e = pub_expo;
    let serialized_n = module;
    let serialized_d = priv_d;

    let val = Pem::new("PUBLIC KEY", serialized_e);
    let public = pem::encode(&val);

    let val = Pem::new("N VALUE", serialized_n);
    let nn = pem::encode(&val);

    let val = Pem::new("PRIVATE KEY", serialized_d);
    let private = pem::encode(&val);

    //file with pub key(e,n)
    let mut file_to_send = File::create(file.unwrap()).unwrap();
    file_to_send.write_all(public.as_bytes()).unwrap();
    file_to_send.write_all(nn.as_bytes()).unwrap();

    // file with pub and prvt key
    let mut config_ = File::create("config").unwrap();
    config_.write_all(public.as_bytes()).unwrap();
    config_.write_all(nn.as_bytes()).unwrap();
    config_.write_all(private.as_bytes()).unwrap();
}
