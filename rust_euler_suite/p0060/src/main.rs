// <p>The primes $3$, $7$, $109$, and $673$, are quite remarkable. By taking any two primes and concatenating them in any order the result will always be prime. For example, taking $7$ and $109$, both $7109$ and $1097$ are prime. The sum of these four primes, $792$, represents the lowest sum for a set of four primes with this property.</p>
// <p>Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.</p>

use primes::{Sieve, PrimeSet};
use lib_prime_functions::concatenating_primes::is_concatenating_prime_for_vec;
use sorted_list::SortedList;

type ConPrimeSet = (u64, Vec<u64>);
type ConPrimeSetList = SortedList<u64, Vec<u64>>;

fn main() {
    // idea:
    // Maintain a list of sets of concatenating primes.
    // This list has to be sorted by the sum of the set in increasing order.
    // Repeat: Generate the next prime number an compare with all previous sets.
    //         If it forms a concatenating prime set then compute the sum and add the 
    //         new set correctly ordered to the list of sets.
    //         If we genereate a set of 5 primes this is the solution and we can stop.

    const MAX_PRIME: u64 = 100000;
    let mut psieve: Sieve = Sieve::new();
    let mut known_concatenating_prime_sets: ConPrimeSetList = SortedList::new();

    for prime in 2..MAX_PRIME {
        if !psieve.is_prime(prime) {
            continue;
        }
        let maybe_5_set = add_next_prime(prime, &mut known_concatenating_prime_sets, &mut psieve);

        if maybe_5_set.is_some() {
            println!("Found 5-tuple of concatenating primes with lowest sum {}: {:?}",
                maybe_5_set.as_ref().unwrap().0,
                maybe_5_set.as_ref().unwrap().1);
            break;
        }
    }
}

fn add_next_prime(prime: u64, known_concatenating_prime_sets: &mut ConPrimeSetList, psieve: &mut Sieve) -> Option<ConPrimeSet> {
    let mut new_sets: ConPrimeSetList = SortedList::new();
    new_sets.insert(prime, vec![prime]);

    for set in known_concatenating_prime_sets.iter() {
        if is_concatenating_prime_for_vec(prime, set.1, psieve) {
            let mut new_set: ConPrimeSet = (
                set.0 + prime,
                set.1.clone()
            );
            new_set.1.push(prime);
            new_sets.insert(new_set.0, new_set.1);
        }
    }

    for new in new_sets {
        if new.1.len() >= 5 {
            return Some(new)
        }
        else if new.1.len() == 4 {
            println!("Found 4-tuple of concatenating primes with sum {}: {:?}",
                new.0,
                new.1)
        }
        known_concatenating_prime_sets.insert(new.0, new.1);
    }

    None
}

// Found 5-tuple of concatenating primes with lowest sum 26033: [13, 5197, 5701, 6733, 8389]
