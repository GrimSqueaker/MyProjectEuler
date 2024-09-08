use std::cmp;
use num_bigint::{BigUint, ToBigUint};

pub fn get_number_of_digits(mut num: u128) -> u64 {
    let mut num_digits = 0;

    while num > 0 {
        num_digits += 1;
        num /= 10;
    }

    num_digits
}

pub fn get_sorted_digits(mut num: u64) -> u64 {
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

pub fn get_n_th_digit(mut n: u64, mut num: u64) -> u64 {
    while n > 0 {
        num /= 10;
        n -= 1;
    }

    num % 10
}

pub fn set_n_th_digit(n: u64, mut num: u64, to: u64) -> u64 {
    let mut exp = 1;
    let len = cmp::max(get_number_of_digits(num.into()), n+1);
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

pub fn get_last_n_digits(mut n: u64, num: u64) -> u64 {
    let mut modul = 1;

    while n > 0 {
        modul *= 10;
        n -= 1;
    }

    num % modul
}

pub fn get_right_rotated_u128(mut n: u128) -> u128 {
    let result = n/10;
    let mut rotated_digit = n%10;

    while n >= 10 {
        rotated_digit *= 10;
        n /= 10;
    }

    result + rotated_digit
}

pub fn get_right_rotated_biguint(n: &BigUint) -> BigUint {
    let ten: BigUint = 10.to_biguint().unwrap();
    let result: BigUint = n/&ten;
    let mut rotated_digit: BigUint = n%&ten;

    let mut temp: BigUint = n.clone();

    while temp >= ten {
        rotated_digit *= &ten;
        temp /= &ten;
    }

    result + rotated_digit
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_digits() {
        assert_eq!(get_number_of_digits(0), 0);
        assert_eq!(get_number_of_digits(10), 2);
        assert_eq!(get_number_of_digits(141), 3);
        assert_eq!(get_number_of_digits(9876), 4);
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
    fn test_get_last_n_digits() {
        assert_eq!(get_last_n_digits(1, 123), 3);
        assert_eq!(get_last_n_digits(4, 123), 123);
        assert_eq!(get_last_n_digits(4, 987123), 7123);
    }

    #[test]
    fn test_get_right_rotated() {
        assert_eq!(get_right_rotated_u128(10), 1);
        assert_eq!(get_right_rotated_u128(1234), 4123);
        assert_eq!(get_right_rotated_u128(142857), 714285);
        assert_eq!(get_right_rotated_u128(102564), 410256);
        assert_eq!(get_right_rotated_biguint(&10.to_biguint().unwrap()), 1.to_biguint().unwrap());
        assert_eq!(get_right_rotated_biguint(&1234.to_biguint().unwrap()), 4123.to_biguint().unwrap());
        assert_eq!(get_right_rotated_biguint(&142857.to_biguint().unwrap()), 714285.to_biguint().unwrap());
        assert_eq!(get_right_rotated_biguint(&102564.to_biguint().unwrap()), 410256.to_biguint().unwrap());
    }
}
