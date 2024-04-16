use primes::{Sieve, PrimeSet};

pub fn concatenate_numbers(left: u64, right: u64) -> u64
{
    let mut right_copy = right;
    let mut result = left;

    while right_copy > 0 {
        right_copy /= 10;
        result *= 10;
    }

    result + right
}

pub fn is_concatenating_prime_pair(prime1: u64, prime2: u64, psieve: &mut Sieve) -> bool
{
    let lr = concatenate_numbers(prime1, prime2);
    let rl = concatenate_numbers(prime2, prime1);

    psieve.is_prime(lr) && psieve.is_prime(rl)
}

pub fn is_concatenating_prime_for_vec(prime: u64, concat_primes: &Vec<u64>, psieve: &mut Sieve) -> bool
{
    // assuming that concat_primes are already pair-wise concatenating
    concat_primes
        .iter()
        .all(|x| is_concatenating_prime_pair(*x, prime, psieve))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_numbers() {
        assert_eq!(concatenate_numbers(3, 7), 37);
        assert_eq!(concatenate_numbers(7, 3), 73);
        assert_eq!(concatenate_numbers(109, 673), 109673);
    }

    #[test]
    fn test_is_concatenating_prime_pair() {
        let mut psieve = Sieve::new();
        assert_eq!(is_concatenating_prime_pair(3, 7, &mut psieve), true);
        assert_eq!(is_concatenating_prime_pair(7, 3, &mut psieve), true);
        assert_eq!(is_concatenating_prime_pair(109, 673, &mut psieve), true);
        assert_eq!(is_concatenating_prime_pair(7, 5, &mut psieve), false);
    }

    #[test]
    fn test_is_concatenating_prime_for_vec() {
        let mut psieve = Sieve::new();
        assert_eq!(is_concatenating_prime_for_vec(3, &vec![7], &mut psieve), true);
        assert_eq!(is_concatenating_prime_for_vec(3, &vec![7,109,673], &mut psieve), true);
        assert_eq!(is_concatenating_prime_for_vec(673, &vec![3,7,109], &mut psieve), true);
        assert_eq!(is_concatenating_prime_for_vec(5, &vec![7,109,673], &mut psieve), false);
    }
}
