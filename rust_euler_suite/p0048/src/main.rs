// Self Powers
// 
// Problem 48
// 
// The series, 1^1 + 2^2 + 3^3 + \cdots + 10^{10} = 10405071317 .
// 
// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + \cdots + 1000^{1000} .

fn main() {
    let res = powers_sum(1000, 10u128.pow(10));

    println!("Solution: {}", res);
}

fn powers_sum(steps: u128, modulus: u128) -> u128 {
    let mut sum = 0u128;

    for s in 1..steps {
        sum += integer_power(s, s, modulus);
        sum %= modulus;
    }

    sum
}

fn integer_power(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    let mut result = 1u128;

    while exponent > 0 {
        if (exponent & 1) == 1 {
            result *= base;
            result %= modulus;
        }

        exponent /= 2;

        base *= base;
        base %= modulus;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_power() {
        assert_eq!(integer_power(1, 1, 1000), 1);
        assert_eq!(integer_power(10, 0, 1000), 1);
        assert_eq!(integer_power(32, 2, 1000), 24);
        assert_ne!(integer_power(10, 2, 1000), 1);
        assert_eq!(integer_power(2, 10, 1000), 24);
    }

    #[test]
    fn test_powers_sum() {
        assert_eq!(powers_sum(10, 10u128.pow(10)), 405071317u128);
    }
}

// Solution: 9110846700
