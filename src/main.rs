use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use std::fs::File;
use std::io::{Read, Write};
use std::str;

fn is_prime(n: &BigInt, k: i32) -> bool {
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

fn generate_large_prime(bits: u32) -> BigInt {
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

fn find_e(phi: &BigInt) -> BigInt {
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

fn mod_inverse(e: &BigInt, phi: &BigInt) -> Option<BigInt> {
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

fn main() {
    let num_bits = 1024;

    let large_prime = generate_large_prime(num_bits);

    let large_prime2 = generate_large_prime(num_bits);

    let n = &large_prime * &large_prime2;

    let phi_n = (&large_prime - BigInt::one()) * (&large_prime2 - BigInt::one());

    let e = find_e(&phi_n);

    // println!("First prime number {}", large_prime);
    // println!("Second prime number{}", large_prime2);
    // println!("N is equals{}", n);
    // println!("ph(N) is equals{}", &phi_n);

    // println!("Generated Large Prime Number not equal: {}", large_prime != large_prime2);

    println!("Public key is {}", e);

    let d = mod_inverse(&e, &phi_n).expect("Error");
    println!("LENYA LOX is {}", d);

    let mut text = "Hello World".as_bytes();
    println!("{:?}", text);

    let encrypted: Vec<BigInt> = text
        .into_iter()
        .map(|x| BigInt::from(*x as i32).modpow(&e, &n))
        .collect();

    // println!("encrypted: {:?}", encrypted);

    let mut peredacha: Vec<u8> = Vec::new();
    for i in encrypted {
        let (sign, mut v) = i.to_bytes_le();
        peredacha.push(match sign {
            num_bigint::Sign::Minus => 0u8,
            num_bigint::Sign::NoSign => 1,
            num_bigint::Sign::Plus => 2,
        });
        let size = (v.len() as u16).to_le_bytes();
        peredacha.extend_from_slice(&size);
        peredacha.append(&mut v);
    }

    let mut file = File::create("peredacha.bin").unwrap();
    file.write_all(&peredacha).unwrap();

    let mut file = File::open("peredacha.bin").unwrap();
    let mut peredacha = vec![];
    file.read_to_end(&mut peredacha).unwrap();
    // file.write_all(&peredacha).unwrap();

    let mut prinyli: Vec<BigInt> = vec![];
    let mut ptr = 0usize;
    let mut zakonchil: bool = false;
    while !zakonchil {
        let sign = peredacha[ptr];
        println!("{}", sign);
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
        // println!("converted to bigint");
        // ptr += 1;
        // println!("ptr {ptr}");
        if ptr == peredacha.len() {
            zakonchil = true;
        }
    }

    // [0][1]   [3]      [11][12]    [14]
    // [0][1..2][3.. 3+8][11][12..13][14.. 14 + 6]

    let decrypted: Vec<BigInt> = prinyli.into_iter().map(|x| x.modpow(&d, &n)).collect();

    for i in decrypted {
        println!("{:?}", str::from_utf8(&i.to_bytes_be().1).unwrap());
    }

    // println!("decrypted: {:?}", decrypted);
    println!("Generated Large Prime Number: {}", d);
    println!("Generated Large Prime Number: {}", large_prime2);
}
