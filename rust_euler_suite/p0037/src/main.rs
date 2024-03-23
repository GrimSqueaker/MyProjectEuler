use primes::{Sieve, PrimeSet};


//The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
//
//Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
//
//NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

fn is_remove_left_prime(p: u64) -> bool {
    let mut check = p;
    let mut magn: u64 = 1;

    if p < 10 {
        return false;
    }

    while magn < p {
        magn *= 10;
    }

    magn /= 10;

    while check > 0 {
        if !primes::is_prime(check) {
            return false;
        }

        check = check % magn;
        magn /= 10;
    }

    return true;
}

fn is_remove_right_prime(p: u64) -> bool {
    let mut check = p;

    if p < 10 {
        return false;
    }

    while check > 0 {
        if !primes::is_prime(check) {
            return false;
        }

        check = check / 10;
    }
    
    return true;
}

fn main() {
    let mut pset = Sieve::new();
    let mut trunc_primes = Vec::new();

    for (ix, prime) in pset.iter().enumerate().take(100000) {
        let is_lt_prime = is_remove_left_prime(prime);
        let is_rt_prime = is_remove_right_prime(prime);
        let is_trunc_prime = is_lt_prime && is_rt_prime;
        if is_trunc_prime {
            trunc_primes.push(prime);
            println!("Prime {}: {} is rt {}, lt {}, truncatable {}", ix, prime, is_rt_prime, is_lt_prime, is_trunc_prime);
        }

        if trunc_primes.len() == 11 {
            break;
        }
    }

    let mut sum: u64 = 0;
    for trunc_prime in &trunc_primes {
        sum += trunc_prime;
    }

    println!("Sum of all 11 truncatable primes: {sum}");
}


// Prime 8: 23 is rt true, lt true, truncatable true
// Prime 11: 37 is rt true, lt true, truncatable true
// Prime 15: 53 is rt true, lt true, truncatable true
// Prime 20: 73 is rt true, lt true, truncatable true
// Prime 64: 313 is rt true, lt true, truncatable true
// Prime 65: 317 is rt true, lt true, truncatable true
// Prime 73: 373 is rt true, lt true, truncatable true
// Prime 138: 797 is rt true, lt true, truncatable true
// Prime 445: 3137 is rt true, lt true, truncatable true
// Prime 527: 3797 is rt true, lt true, truncatable true
// Prime 59488: 739397 is rt true, lt true, truncatable true
// Sum of all 11 truncatable primes: 748317
// 