// https://projecteuler.net/problem=43
// 
// Sub-string Divisibility
// 
// Problem 43
// 
// The number, , is a to pandigital number because it is made up of each of the digits to
// in some order, but it also has a rather interesting sub-string divisibility property.
// Let
// be the st digit, be the
// nd digit, and so on. In this way, we note the following:
// is divisible by is divisible by is divisible by is divisible by is divisible by is divisible by is divisible by Find the sum of all to pandigital numbers with this property.
// 

use itertools::Itertools;


fn main() {
    let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // tuples: start, length, divisor
    let checks = vec![
        (1, 3, 2),
        (2, 3, 3),
        (3, 3, 5),
        (4, 3, 7),
        (5, 3, 11),
        (6, 3, 13),
        (7, 3, 17),
    ];

    let mut num_found = 0;
    let mut sum: i64 = 0;

    for perm in items.iter().permutations(items.len()).unique() {
        let mut found = true;
        for check in &checks {
            let mut num = 0;
            for item in check.0..(check.0 + check.1) {
                num *= 10;
                num += perm[item]
            }
            if num % check.2 != 0 {
                found = false;
                break;
            }
        }
        if found == true {
            println!("found: {:?}", perm);
            num_found += 1;

            let mut num: i64 = 0;
            for item in 0..10 {
                num *= 10;
                num += perm[item]
            }

            sum += num;
        }
    }

    println!("num_found: {:?}", num_found);
    println!("sum: {:?}", sum);
}

// found: [1, 4, 0, 6, 3, 5, 7, 2, 8, 9]
// found: [1, 4, 3, 0, 9, 5, 2, 8, 6, 7]
// found: [1, 4, 6, 0, 3, 5, 7, 2, 8, 9]
// found: [4, 1, 0, 6, 3, 5, 7, 2, 8, 9]
// found: [4, 1, 3, 0, 9, 5, 2, 8, 6, 7]
// found: [4, 1, 6, 0, 3, 5, 7, 2, 8, 9]
// num_found: 6
// sum: 16695334890
