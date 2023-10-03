use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use std::{error::Error, fs::File, io::Write};


pub fn is_prime(n: &BigInt, k: i32) -> bool {
    if *n <= BigInt::one() {
        return false;
    }
    if *n <= BigInt::from(3i32) {
        return true;
    }

    if n.is_multiple_of(&BigInt::from(2i32)) || n.is_multiple_of(&BigInt::from(3i32)) {
        return false;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a = rng.gen_bigint_range(&BigInt::from(2i32), &(n - BigInt::from(2i32)));

        if a.modpow(&(n - &BigInt::one()), n) != BigInt::one() {
            return false;
        }
    }

    true
}

pub fn xor(p_s_key: &[u8], messege: &[u8]) -> Vec<u8> {
    let mut key = p_s_key.iter().cycle();
    let new_mes = messege.iter().map(|x| x ^ key.next().unwrap()).collect();
    new_mes
}

pub fn generate_large_prime(bits: u32) -> BigInt {
    let mut rng = rand::thread_rng();

    let mut candidate: BigInt;

    loop {
        candidate = rng.gen_bigint(bits as usize);
        if candidate.is_even() {
            candidate += BigInt::one();
        }
        if is_prime(&candidate, 5) {
            break;
        }
    }

    candidate
}

pub fn find_e(phi: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();

    let mut e = rng.gen_bigint_range(&BigInt::from(3i32), &(phi - BigInt::one()));

    let e = loop {
        if e.gcd(phi) == BigInt::one() {
            break e;
        } else {
            e += BigInt::one();
        }
    };
    e
}
//Находим обратное по модулю Пример - ax = 1 (mod phi_n) , мы на ходим x - что является нашим приватным ключом
pub fn mod_inverse(e: &BigInt, phi: &BigInt) -> Option<BigInt> {
    let zero = BigInt::zero();
    let one = BigInt::one();

    let (mut x, mut y, mut x1, mut y1) = (zero.clone(), one.clone(), one.clone(), zero.clone());
    let (mut a, mut b) = (phi.clone(), e.clone());

    while !b.is_zero() {
        let q = &a / &b;
        let new_a = b.clone();
        b = &a % &b;
        a = new_a;

        let new_x = x.clone() - &q * x1.clone();
        let new_y = y.clone() - &q * y1.clone();
        x = x1;
        y = y1;
        x1 = new_x;
        y1 = new_y;
    }

    if a != one {
        None // Нет обратного элемента
    } else {
        if x < zero {
            x += phi;
        }
        Some(x)
    }
}

pub fn processing(bit_length: u32) -> Result<(BigInt, BigInt, BigInt), Box<dyn Error>> {
    let num_bits = bit_length;

    let large_prime = generate_large_prime(num_bits);

    let large_prime2 = generate_large_prime(num_bits);

    let n = &large_prime * &large_prime2;

    let phi_n = (&large_prime - BigInt::one()) * (&large_prime2 - BigInt::one());

    let e = find_e(&phi_n);

    //Private key
    let d = mod_inverse(&e, &phi_n).expect("Error");

    // println!("Public key is {}", e);
    //
    // println!("Private key is {}", d);

    Ok((e, d, n))
}

pub fn encryptinon(e: &BigInt, n: &BigInt, messege: &[u8]) -> Result<Vec<BigInt>, Box<dyn Error>> {
    let encrypted: Vec<BigInt> = messege
        .into_iter()
        .map(|x| BigInt::from(*x as i32).modpow(&e, &n))
        .collect();
    Ok(encrypted)
}
pub fn serialization(enc: Vec<BigInt>) -> Result<(Vec<u8>), Box<dyn Error>> {
    let mut ser_enc: Vec<u8> = Vec::new();
    for i in enc {
        let (sign, mut v) = i.to_bytes_le();
        ser_enc.push(match sign {
            num_bigint::Sign::Minus => 0u8,
            num_bigint::Sign::NoSign => 1,
            num_bigint::Sign::Plus => 2,
        });
        let size = (v.len() as u16).to_le_bytes();
        ser_enc.extend_from_slice(&size);
        ser_enc.append(&mut v);
    }
    Ok(ser_enc)
}

pub fn deseriallization(peredacha: Vec<u8>) -> Result<Vec<BigInt>, Box<dyn Error>> {
    let mut prinyli: Vec<BigInt> = vec![];
    let mut ptr = 0usize;
    let mut zakonchil: bool = false;
    while !zakonchil {
        let sign = peredacha[ptr];
        // println!("{}", sign);
        let sign = match sign {
            0 => num_bigint::Sign::Minus,
            1 => num_bigint::Sign::NoSign,
            2 => num_bigint::Sign::Plus,
            _ => panic!("Gde to ya proebalsya"),
        };
        ptr += 1;
        // println!("ptr {ptr}");
        let size_bytes = &peredacha[ptr..ptr + 2];
        let mut size_bytes2 = [0u8; 2];
        for (idx, byte) in size_bytes.iter().enumerate() {
            size_bytes2[idx] = *byte;
        }
        ptr += 2;
        // println!("ptr {ptr}");
        let size = u16::from_le_bytes(size_bytes2);
        // println!("size {size}");
        // println!("ptr {ptr}");
        let bytes = &peredacha[ptr..(ptr + size as usize)];
        ptr += size as usize;
        let bigint = BigInt::from_bytes_le(sign, bytes);
        prinyli.push(bigint);
        if ptr == peredacha.len() {
            zakonchil = true;
        }
    }
    //hello

    Ok(prinyli)
}

pub fn decryption(desir: Vec<BigInt>, d: BigInt, n: BigInt) -> Result<Vec<BigInt>, Box<dyn Error>> {
    let decrypted: Vec<BigInt> = desir.into_iter().map(|x| x.modpow(&d, &n)).collect();
    Ok(decrypted)
}
