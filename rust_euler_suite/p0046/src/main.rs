// Goldbach's Other Conjecture
// 
// Problem 46
// 
// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.
//
//   \begin{align}
//   9 = 7 + 2 \times 1^2\\
//   15 = 7 + 2 \times 2^2\\
//   21 = 3 + 2 \times 3^2\\
//   25 = 7 + 2 \times 3^2\\
//   27 = 19 + 2 \times 2^2\\
//   33 = 31 + 2 \times 1^2
//   \end{align}
// 
// It turns out that the conjecture was false.
// 
// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

use primes::{Sieve, PrimeSet};

fn main() {
    const MAX: u64 = 10000000;
    let mut pset = Sieve::new();

    // pre-generate prime sieve up to MAX
    //let (idx, prime) = pset.find(MAX);
    //println!("Next prime after {} is {} and has index {}", MAX, prime, idx);

    for test in (3..MAX).step_by(2) {
        // only consider composites - skip primes
        if pset.is_prime(test) {
            continue;
        }

        // check all prime-twice-square-sums
        let mut found = false;
        'outer: for prime in pset.iter().take_while(|n| *n < test) {
            'inner: for s in 1..test {
                let check = prime + 2*s*s;
                if test == check {
                    found = true;
                    println!("{} = {} + 2*{}^2", test, prime, s);
                    break 'outer;
                }
                else if test < check {
                    break 'inner;
                }
            }
        }

        if found == false {
            println!("Smallest odd composite that cannot be written as prime plus twice square: {}", test);
            break;
        }
    }
}

// Smallest odd composite that cannot be written as prime plus twice square: 5777
