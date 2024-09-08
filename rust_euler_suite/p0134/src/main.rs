//
// <p>Consider the consecutive primes $p_1 = 19$ and $p_2 = 23$. It can be verified that $1219$ is the smallest number such that the last digits are formed by $p_1$ whilst also being divisible by $p_2$.</p>
// <p>In fact, with the exception of $p_1 = 3$ and $p_2 = 5$, for every pair of consecutive primes, $p_2 \gt p_1$, there exist values of $n$ for which the last digits are formed by $p_1$ and $n$ is divisible by $p_2$. Let $S$ be the smallest of these values of $n$.</p>
// <p>Find $\sum S$ for every pair of consecutive primes with $5 \le p_1 \le 1000000$.</p>
// 

use primes::{Sieve, PrimeSet};
use lib_number_functions::number_functions::{get_number_of_digits, get_last_n_digits};

fn main() {
    const MAX_PRIME: u64 = 1000000;

    // start with 5
    let mut psieve: Sieve = Sieve::new();
    let mut primeit = psieve.iter();
    primeit.next();
    let mut last: u64;
    let mut next = primeit.next().unwrap();

    let mut sum = 0u128;

    while next < MAX_PRIME {
        last = next;
        next = primeit.next().unwrap();
        let temp = get_smallest_p1_digits_p2_divisible(last, next);
        sum += temp as u128;
        println!("pair: {}, {} -> {}", last, next, temp);
    }

    println!("Sum of all S: {}", sum);
}

fn get_smallest_p1_digits_p2_divisible(p1: u64, p2: u64) -> u64 {
    let mut check = 0u64;
    let digits = get_number_of_digits(p1.into());

    loop {
        check += p2;

        if p1 == get_last_n_digits(digits, check) {
            return check;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_smallest_p1_digits_p2_divisible() {
        assert_eq!(get_smallest_p1_digits_p2_divisible(19, 23), 1219);
        assert_eq!(get_smallest_p1_digits_p2_divisible(5, 7), 35);
    }
}

// Sum of all S: 18613426663617118
