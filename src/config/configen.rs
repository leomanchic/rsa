use serde::Serialize;
use std::{error::Error, fs::File, io::Write};
use toml;

#[derive(Serialize)]
struct Config {
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    blowfish: Blowfish_key,
    rsa: RSA,
}
#[derive(Serialize)]
struct Blowfish_key {
    private_key: String,
}
#[derive(Serialize)]
struct RSA {
    public_key: String,
    private_key: String,
}

// #[warn(unused_must_use)]
pub fn toml_gen(pub_key: String, pr_key: String, bl_key: String) {
    let conf = Config {
        keys: Keys {
            blowfish: Blowfish_key {
                private_key: bl_key,
            },
            rsa: RSA {
                public_key: pub_key,
                private_key: pr_key,
            },
        },
    };
    let mut file = File::create("config.toml").unwrap();
    let toml = toml::to_string(&conf).unwrap();
    file.write_all(&toml.as_bytes());
}
