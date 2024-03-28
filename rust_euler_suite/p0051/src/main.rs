// Prime Digit Replacements
//
// Problem 51
//
// By replacing the 1st digit of the 2-digit number *3,
// it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
//
// By replacing the 3rd and 4th digits of 56**3 with the same digit,
// this 5-digit number is the first example having seven primes among the ten generated numbers,
// yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
// Consequently 56003, being the first member of this family, is the smallest prime with this property.
//
// Find the smallest prime which, by replacing part of the number
// (not necessarily adjacent digits) with the same digit,
// is part of an eight prime value family.

use primes::{Sieve, PrimeSet};

fn main() {
    for l in 2..10 {
        let repla = find_prime_digit_replacements_of_length(l);

        println!("Longest replacement in {}-digit primes is {:?} with length {}", l, repla, repla.len());

        if repla.len() >= 8 {
            println!("Solution {}", repla[0]);
            break;
        }
    }
}

fn find_prime_digit_replacements_of_length(length: u64) -> Vec<u64> {
    let mut pset = Sieve::new();
    let upper_limit = 10u64.pow(length as u32);
    pset.find(upper_limit); // pre-compute
    let mut pset_check = pset.clone();

    let mut longest_replacement: Vec<u64> = vec![];

    let masks = generate_masks_for_length(length);

    for prime in pset.iter().take_while(|n| *n < upper_limit) {
        if prime < 10u64.pow(length as u32 - 1) {
            continue;
        }

        let repla = check_families(prime, &masks, &mut pset_check);
        if repla.len() > longest_replacement.len() {
            longest_replacement = repla;
            println!("longest replacement yet {:?}", longest_replacement);
        }
    }

    return longest_replacement;
}

fn generate_masks_for_length(length: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];

    for i in 1..(2u32.pow(length as u32 - 1)) {
        let mut bitmask = i;
        let mut pos = 10;
        let mut mask = 0;

        while bitmask > 0 {
            if bitmask % 2 == 1 {
                mask += pos;
            }

            pos *= 10;
            bitmask /= 2;
        }

        result.push(mask);
    }

    return result;
}

fn check_families(num: u64, masks: &Vec<u64>, pset: &mut Sieve) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];

    for mask in masks.iter() {
        let mut mask_result : Vec<u64>= vec![];
        if !number_has_same_digits_at_mask(num, *mask) {
            continue;
        }

        let num_zero_at_mask = number_with_zeros_at_mask(num, *mask);

        for i in 0..=9 {
            let num_test = num_zero_at_mask + i*mask;
            if num <= num_test && pset.is_prime(num_test) {
                mask_result.push(num_test);
            }
        }

        if mask_result.len() > result.len() {
            result = mask_result;
        }
    }

    return result;
}

fn number_has_same_digits_at_mask(mut num: u64, mut mask: u64) -> bool {
    let mut digit: i32 = -1;

    while mask > 0 {
        if mask % 10 == 1 {
            if digit == -1 {
                digit = (num % 10) as i32;
            }
            else {
                if digit != (num % 10) as i32 {
                    return false;
                }
            }
        }

        mask /= 10;
        num /= 10;
    }

    return true;
}

fn number_with_zeros_at_mask(mut num: u64, mut mask: u64) -> u64 {
    let mut pos = 1;
    let mut result = 0;

    while num > 0 {
        if mask % 10 == 0 {
            result += pos * (num%10);
        }

        num /= 10;
        mask /= 10;
        pos *= 10;
    }

    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prime_digit_replacements_of_length() {
        assert_eq!(find_prime_digit_replacements_of_length(2), vec![13,23,43,53,73,83]);
        assert_eq!(find_prime_digit_replacements_of_length(5), vec![56003, 56113, 56333, 56443, 56663, 56773, 56993]);
    }

    #[test]
    fn test_generate_masks_for_length() {
        assert_eq!(generate_masks_for_length(2), vec![10]);
        assert_eq!(generate_masks_for_length(3), vec![010, 100, 110]);
        assert_eq!(generate_masks_for_length(4), vec![0010, 0100, 0110, 1000, 1010, 1100, 1110]);
    }

    #[test]
    fn test_number_has_same_digits_at_mask() {
        assert_eq!(number_has_same_digits_at_mask(1234, 1100), false);
        assert_eq!(number_has_same_digits_at_mask(1234, 0100), true);
        assert_eq!(number_has_same_digits_at_mask(10001, 01010), true);
        assert_eq!(number_has_same_digits_at_mask(56003, 110), true);
    }

    #[test]
    fn test_number_with_zeros_at_mask() {
        assert_eq!(number_with_zeros_at_mask(1233, 0011), 1200);
        assert_eq!(number_with_zeros_at_mask(1224, 0110), 1004);
    }
}

// longest replacement yet [11, 31, 41, 61, 71]
// longest replacement yet [13, 23, 43, 53, 73, 83]
// Longest replacement in 2-digit primes is [13, 23, 43, 53, 73, 83] with length 6
// longest replacement yet [101, 131, 151, 181, 191]
// longest replacement yet [107, 127, 137, 157, 167, 197]
// Longest replacement in 3-digit primes is [107, 127, 137, 157, 167, 197] with length 6
// longest replacement yet [1009, 1229, 1559, 1669, 1889, 1999]
// Longest replacement in 4-digit primes is [1009, 1229, 1559, 1669, 1889, 1999] with length 6
// longest replacement yet [10007, 11117, 12227, 13337, 14447, 19997]
// longest replacement yet [56003, 56113, 56333, 56443, 56663, 56773, 56993]
// Longest replacement in 5-digit primes is [56003, 56113, 56333, 56443, 56663, 56773, 56993] with length 7
// longest replacement yet [100003, 101113, 103333, 106663, 107773, 108883]
// longest replacement yet [111109, 222109, 444109, 555109, 666109, 777109, 888109]
// longest replacement yet [121313, 222323, 323333, 424343, 525353, 626363, 828383, 929393]
// Longest replacement in 6-digit primes is [121313, 222323, 323333, 424343, 525353, 626363, 828383, 929393] with length 8
// Solution 121313
