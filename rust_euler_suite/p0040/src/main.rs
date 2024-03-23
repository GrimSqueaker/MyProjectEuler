// Champernowne's constant
// 
// Problem 40
// 
// An irrational decimal fraction is created by concatenating the positive integers:
// 
// 0.123456789101112131415161718192021...
// 
// It can be seen that the 12th digit of the fractional part is 1.
// 
// If dn represents the nth digit of the fractional part, find the value of the following expression.
// 
// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000


fn main() {
    let mut current_digit_num = 1;
    let mut current_integer: u64 = 1;

    let to_mult_digits: Vec<u64> = vec![1, 10, 100, 1000, 10000, 100000, 1000000];

    let mut prod = 1;

    const MAX: u64 = 1000000;

    while current_digit_num < MAX {
        let next_digits = current_integer.ilog10() + 1;
        
        let mut test = current_integer;
        for i in (0..next_digits).rev() {
            let digit = test / 10u64.pow(i);
            test = test - digit*10u64.pow(i);

            // check
            if to_mult_digits.contains(&current_digit_num) {
                prod *= digit;
            }

            current_digit_num += 1;
        }

        current_integer += 1;
    }

    println!("The product {}", prod);
}

//The product 210