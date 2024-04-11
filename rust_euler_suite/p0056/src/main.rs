// <p>A googol ($10^{100}$) is a massive number: one followed by one-hundred zeros; $100^{100}$ is almost unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the digits in each number is only $1$.</p>
// <p>Considering natural numbers of the form, $a^b$, where $a, b \lt 100$, what is the maximum digital sum?</p>

use num_bigint::ToBigUint;

fn main() {

    let mut max_sum = 0u32;

    for a in 1..100 {
        for b in 1..100 {
            let sum = get_digit_sum(a, b);

            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    println!("Maximum digit sum: {}", max_sum);
}

fn get_digit_sum(a: u32, b: u32) -> u32 {
    let aa = a.to_biguint().unwrap();
    let mut pow = aa.pow(b);

    let mut digitsum = 0;

    while pow > 0.to_biguint().unwrap() {
        let powmod10 = pow.clone() % 10.to_biguint().unwrap();
        digitsum = digitsum + powmod10.iter_u32_digits().next().unwrap_or(0);
        pow /= 10.to_biguint().unwrap();
    }

    digitsum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digital_sum() {
        assert_eq!(get_digit_sum(1,1), 1);
        assert_eq!(get_digit_sum(2,2), 4);
        assert_eq!(get_digit_sum(2,8), 13);
        assert_eq!(get_digit_sum(2,16), 25);
    }
}

// Maximum digit sum: 972
