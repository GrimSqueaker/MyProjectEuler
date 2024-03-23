// Pandigital prime
// 
// Problem 41
// 
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
// 
// What is the largest n-digit pandigital prime that exists?

use primes::{Sieve, PrimeSet};


fn is_pandigital(number: u64) -> bool {
    if number > 999999999 {
        return false;
    }

    let len: usize = (number.ilog10() + 1) as usize;
    let mut n = number;
    let mut present = [false; 10];

    while n > 0 {
        let digit = (n % 10) as usize;
        n /= 10;

        if present[digit] == true {
            return false;
        }
        else {
            present[digit] = true;
        }
    }

    if present[0] {
        return false;
    }

    for i in 1..(len+1) {
        if !present[i] {
            return false;
        }
    }

    return true;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital() {
        assert_eq!(is_pandigital(1), true);
        assert_eq!(is_pandigital(12), true);
        assert_eq!(is_pandigital(21), true);
        assert_eq!(is_pandigital(11), false);
        assert_eq!(is_pandigital(23), false);
        assert_eq!(is_pandigital(213), true);
        assert_eq!(is_pandigital(123456789), true);
        assert_eq!(is_pandigital(193456782), true);
        assert_eq!(is_pandigital(113456782), false);
        assert_eq!(is_pandigital(8654231), false);
    }
}

fn main() {
    // Note: Every 9-pandigital number has a digit sum divisible by 3 and cannot be a prime,
    //       because sum_{i=1}^9(i) = 45.
    //       The same argument holds for all 8-pandigitals: sum_{i=1}^8(i) = 36.
    //       So, the largest pandigital prime can only have 7 digits

    const MAX: u64 = 10000000;
    let mut pset = Sieve::new();
    let mut largest = 2;

    // pre-generate prime sieve up to MAX
    //let (idx, prime) = pset.find(MAX);
    //println!("Next prime after {} is {} and has index {}", MAX, prime, idx);

    for test in 2..MAX {
        if pset.is_prime(test) && is_pandigital(test) {
            largest = test;
        }
    }
    println!("The number {} is prime and pandigital", largest);
}

//The number 7652413 is prime and pandigital
