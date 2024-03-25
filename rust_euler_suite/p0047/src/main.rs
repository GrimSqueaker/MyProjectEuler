// Distinct Primes Factors
// 
// Problem 47
// 
// The first two consecutive numbers to have two distinct prime factors are:
//   14 &= 2 \times 7\\
//   15 &= 3 \times 5.
// 
// The first three consecutive numbers to have three distinct prime factors are:
//   644 &= 2^2 \times 7 \times 23\\
//   645 &= 3 \times 5 \times 43\\
//   646 &= 2 \times 17 \times 19.
// 
// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?

use prime_factorization::Factorization;

fn main() {
    const MAX: u32 = 10000000;

    let mut last_4 = 2;

    for i in vec![643, 644, 645, 646, 647] {
        let factor_repr = Factorization::run(i as u32);
        println!("{} {:?}", i, factor_repr.factors);
    }

    for test in last_4..MAX {
        let factor_repr = Factorization::run(test);
        let mut factors = factor_repr.factors;
        factors.dedup();

        if factors.len() < 4 {
            last_4 = test+1;
        }
        else if test - last_4 >= 3 {
            println!("First four consecutive 4-primefactor numbers start at {}", last_4);
            break;
        }
    }
}

// First four consecutive 4-primefactor numbers start at 134043
