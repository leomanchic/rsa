use num_bigint::{BigInt, RandBigInt};
use num_traits::{One,Zero};
/// Denis pidor

use num_integer::{Integer};

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

fn generate_large_prime(bits: u64) -> BigInt {

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

fn find_e(phi:&BigInt) -> BigInt{

    let mut rng = rand::thread_rng();

    let mut e = rng.gen_bigint_range(&BigInt::from(3i32),&BigInt::from(100i32));

    let e = loop {
        if e.gcd(phi) == BigInt::one(){
          
            break e;
        
        } else{

            e+=BigInt::one();
        
        }

    };
    e
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let zero = BigInt::zero();
    let one = BigInt::one();

    if *a == zero {
        return (b.clone(), zero.clone(), one.clone());
    }

    let (gcd, x1, y1) = extended_gcd(&(b % a), a);

    let x = y1.clone() - &(b / a) * x1.clone();

    let y = x1.clone();

    (gcd, x, y)
}



fn main() {
    let num_bits = 512;
    
    let large_prime = generate_large_prime(num_bits);

    let large_prime2 = generate_large_prime(num_bits);

    let phi_n = (&large_prime-BigInt::one())*(&large_prime2 - BigInt::one());
    
    let e = find_e(&phi_n);

    println!("Generated Large Prime Number: {}", large_prime);
    
    println!("Generated Large Prime Number: {}", large_prime2);
    
    println!("Generated Large Prime Number: {}", phi_n);
    
    println!("Generated e: {}", e);

    println!("{}", is_prime(&large_prime, 5));
    
    println!("{}", is_prime(&large_prime2, 5));
    
    let (gcd, x, y) = extended_gcd(&e, &phi_n);
    println!("GCD of {} and {} is: {}", e, phi_n, gcd);
    println!("x: {}, y: {}", x, y);

    // let d = find_modular_inverse(&e,&phi_n).expect("Error");
    // println!("Generated Large Prime Number: {}", d);
    // println!("Generated Large Prime Number: {}", large_prime2);
    
}