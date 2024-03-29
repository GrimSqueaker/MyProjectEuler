// Combinatoric Selections
// 
// Problem 53
// 
// There are exactly ten ways of selecting three from five, 12345:
//   123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
// 
// In combinatorics, we use the notation, $\displaystyle \binom 5 3 = 10$.
// 
// In general, $\displaystyle \binom n r = \dfrac{n!}{r!(n-r)!}$, 
// where
//   $r \le n$, 
//   $n! = n \times (n-1) \times ... \times 3 \times 2 \times 1$, and 
//   $0! = 1$.
// 
// It is not until $n = 23$, that a value exceeds one-million: 
//   $\displaystyle \binom {23} {10} = 1144066$.
//
// How many, not necessarily distinct, values of 
//   $\displaystyle \binom n r$ for $1 \le n \le 100$, 
// are greater than one-million?

use num_integer::binomial;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    let threshold: BigUint = 1000000.to_biguint().unwrap();
    let mut greater_than = 0;

    for n in 1..=100 {
        for r in 1..=n {
            if binomial(n.to_biguint().unwrap(), r.to_biguint().unwrap()) > threshold {
                greater_than += 1;
            }
        }
    }

    println!("Binomials greater than {}: {}", threshold, greater_than);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5u64.to_biguint().unwrap(), 3u64.to_biguint().unwrap()), 10u64.to_biguint().unwrap());
        assert_eq!(binomial(23u64.to_biguint().unwrap(), 10u64.to_biguint().unwrap()), 1144066u64.to_biguint().unwrap());
    }
}

// Binomials greater than 1000000: 4075
