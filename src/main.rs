use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};

use num_integer::Integer;

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

        let new_x = x.clone() -&q * x1.clone();
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
    let num_bits = 16;

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

    let d =  mod_inverse(& e,& phi_n).expect("Error");

    let mut text = "Hello World".as_bytes();
    println!("{:?}", text);

    let encrypted: Vec<BigInt> = text
        .into_iter()
        .map(|x| BigInt::from(*x as i32).modpow(&e, &n))
        .collect();

    println!("encrypted: {:?}", encrypted);

    let decrypted: Vec<BigInt> = encrypted.into_iter().map(|x| x.modpow(&d, &n)).collect();
    
    println!("decrypted: {:?}", decrypted);
    println!("Generated Large Prime Number: {}", d);
    println!("Generated Large Prime Number: {}", large_prime2);
}
