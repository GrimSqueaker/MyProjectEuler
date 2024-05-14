//
// <p>Consider the consecutive primes $p_1 = 19$ and $p_2 = 23$. It can be verified that $1219$ is the smallest number such that the last digits are formed by $p_1$ whilst also being divisible by $p_2$.</p>
// <p>In fact, with the exception of $p_1 = 3$ and $p_2 = 5$, for every pair of consecutive primes, $p_2 \gt p_1$, there exist values of $n$ for which the last digits are formed by $p_1$ and $n$ is divisible by $p_2$. Let $S$ be the smallest of these values of $n$.</p>
// <p>Find $\sum S$ for every pair of consecutive primes with $5 \le p_1 \le 1000000$.</p>
// 

use std::cmp;
use primes::{Sieve, PrimeSet};

fn main() {
    const MAX_PRIME: u64 = 1000000;

    // start with 5
    let mut psieve: Sieve = Sieve::new();
    let mut primeit = psieve.iter();
    primeit.next();
    let mut last = primeit.next().unwrap();
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
    let digits = get_num_digits(p1);

    loop {
        check += p2;

        if p1 == get_last_n_digits(digits, check) {
            return check;
        }
    }
}

fn get_num_digits(mut num: u64) -> u64 {
    let mut dig = 0;
    while num > 0 {
        num /= 10;
        dig += 1;
    }
    dig
}

fn get_last_n_digits(mut n: u64, num: u64) -> u64 {
    let mut modul = 1;

    while n > 0 {
        modul *= 10;
        n -= 1;
    }

    num % modul
}

fn get_sorted_digits(mut num: u64) -> u64 {
    let mut buckets = [0usize; 10];

    // bucketsort
    while num > 0 {
        let dig = num%10;
        num /= 10;
        buckets[dig as usize] += 1;
    }

    for dig in (0..10).rev() {
        while buckets[dig] > 0 {
            num = num*10 + dig as u64;
            buckets[dig] -= 1;
        }
    }

    num
}

fn get_n_th_digit(mut n: u64, mut num: u64) -> u64 {
    while n > 0 {
        num /= 10;
        n -= 1;
    }

    num % 10
}

fn set_n_th_digit(n: u64, mut num: u64, to: u64) -> u64 {
    let mut exp = 1;
    let len = cmp::max(get_num_digits(num), n+1);
    let mut out = 0u64;

    for pos in 0..len {
        let mut dig = num % 10;
        num /= 10;

        if pos == n {
            dig = to;
        }

        out += exp*dig;

        exp *= 10;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_digits() {
        assert_eq!(get_num_digits(0), 0);
        assert_eq!(get_num_digits(7), 1);
        assert_eq!(get_num_digits(10), 2);
        assert_eq!(get_num_digits(141), 3);
    }

    #[test]
    fn test_get_last_n_digits() {
        assert_eq!(get_last_n_digits(1, 123), 3);
        assert_eq!(get_last_n_digits(4, 123), 123);
        assert_eq!(get_last_n_digits(4, 987123), 7123);
    }

    #[test]
    fn test_get_sorted_digits() {
        assert_eq!(get_sorted_digits(1), 1);
        assert_eq!(get_sorted_digits(41), 41);
        assert_eq!(get_sorted_digits(5529), 9552);
        assert_eq!(get_sorted_digits(1000), 1000);
        assert_eq!(get_sorted_digits(103319), 933110);
    }

    #[test]
    fn test_get_n_th_digit() {
        assert_eq!(get_n_th_digit(0, 12), 2);
        assert_eq!(get_n_th_digit(0, 10), 0);
        assert_eq!(get_n_th_digit(1, 10), 1);
        assert_eq!(get_n_th_digit(2, 10), 0);
        assert_eq!(get_n_th_digit(4, 6543210), 4);
    }

    #[test]
    fn test_set_n_th_digit() {
        assert_eq!(set_n_th_digit(0, 12, 3), 13);
        assert_eq!(set_n_th_digit(0, 10, 0), 10);
        assert_eq!(set_n_th_digit(1, 10, 4), 40);
        assert_eq!(set_n_th_digit(3, 10, 5), 5010);
        assert_eq!(set_n_th_digit(4, 6543210, 9), 6593210);
    }

    #[test]
    fn test_get_smallest_p1_digits_p2_divisible() {
        assert_eq!(get_smallest_p1_digits_p2_divisible(19, 23), 1219);
        assert_eq!(get_smallest_p1_digits_p2_divisible(5, 7), 35);
    }
}

// Sum of all S: 18613426663617118
