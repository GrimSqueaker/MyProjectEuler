// Prime Permutations
// 
// Problem 49
// 
// The arithmetic sequence, 1487, 4817, 8147 , in which each of the terms increases by 3330, is unusual in two ways:
//   (i)  each of the three terms are prime, and, 
//   (ii) each of the 4-digit numbers are permutations of one another.
// 
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property,
// but there is one other 4-digit increasing sequence.
// 
// What 12-digit number do you form by concatenating the three terms in this sequence?

use primes::{Sieve, PrimeSet, is_prime};

fn main() {
    let mut pset = Sieve::new();

    for prime in pset.iter().take_while(|n| *n < 10000) {
        if prime < 1000 {
            continue;
        }

        for prime2 in (prime+2)..(prime+(10000-prime)/2 +1) {
            if is_prime(prime2) {
                let prime3 = 2*prime2-prime;
                if is_permutation(prime, prime2) && is_prime(prime3) && is_permutation(prime, prime3) {
                    println!("Solution {}{}{}", prime, prime2, prime3);
                }

            }
            else {
                continue;
            }
        }
    }

    //println!("Solution: {}", res);
}

fn is_permutation(a: u64, b: u64) -> bool {
    let mut a_vec = int_to_digit_vec(a);
    let mut b_vec = int_to_digit_vec(b);

    a_vec.sort();
    b_vec.sort();

    let matching = a_vec.iter().zip(b_vec.iter()).filter(|&(a, b)| a == b).count();
    return matching == a_vec.len() && matching == b_vec.len();
}

fn int_to_digit_vec(mut integer: u64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();

    while integer > 0 {
        v.push(integer % 10);
        integer /= 10;
    }

    v.reverse();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(is_permutation(1487, 4817), true);
        assert_eq!(is_permutation(1007, 7100), true);
        assert_eq!(is_permutation(487, 4817), false);
    }

    #[test]
    fn test_int_to_digit_vec() {
        assert_eq!(int_to_digit_vec(123), vec![1,2,3]);
        assert_eq!(int_to_digit_vec(1002), vec![1,0,0,2]);
    }
}

// Solution 148748178147
// Solution 296962999629
