
// <p>The $5$-digit number, $16807=7^5$, is also a fifth power. Similarly, the $9$-digit number, $134217728=8^9$, is a ninth power.</p>
// <p>How many $n$-digit positive integers exist which are also an $n$th power?</p>

// Notes:
// - 10^n always has n+1 digits, for positive integers n
// - as soon as 9^n has less than n digits, we can stop, because
//                             9^x < 10^(x-1)
//       <=>               ln(9^x) < ln(10^(x-1))
//       <=>               x*ln(9) < (x-1)*ln(10)
//       <=>                ln(10) < x*ln(10)-x*ln(9)
//       <=>                ln(10) < x*(ln(10)-ln(9))
//       <=> ln(10)/(ln(10)-ln(9)) < x
//       <=>               ~21.854 < x

fn main() {
    let mut num_n_digits_n_power = 0;

    for n in 1..=22u64 {
        for x in 1..10u128 {
            let digits = get_number_of_digits(x.pow(n as u32) as u128);
            if n == digits {
                println!("{}^{}={} has {} digits", x, n, x.pow(n as u32), digits);
                num_n_digits_n_power += 1;
            }
        }
    }

    println!("Solution: {}", num_n_digits_n_power);
}

fn get_number_of_digits(mut num: u128) -> u64 {
    let mut num_digits = 0;

    while num > 0 {
        num_digits += 1;
        num /= 10;
    }

    num_digits
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_digits() {
        assert_eq!(get_number_of_digits(10), 2);
        assert_eq!(get_number_of_digits(0), 0);
        assert_eq!(get_number_of_digits(9876), 4);
    }
}

// 1^1=1 has 1 digits
// 2^1=2 has 1 digits
// 3^1=3 has 1 digits
// 4^1=4 has 1 digits
// 5^1=5 has 1 digits
// 6^1=6 has 1 digits
// 7^1=7 has 1 digits
// 8^1=8 has 1 digits
// 9^1=9 has 1 digits
// 4^2=16 has 2 digits
// 5^2=25 has 2 digits
// 6^2=36 has 2 digits
// 7^2=49 has 2 digits
// 8^2=64 has 2 digits
// 9^2=81 has 2 digits
// 5^3=125 has 3 digits
// 6^3=216 has 3 digits
// 7^3=343 has 3 digits
// 8^3=512 has 3 digits
// 9^3=729 has 3 digits
// 6^4=1296 has 4 digits
// 7^4=2401 has 4 digits
// 8^4=4096 has 4 digits
// 9^4=6561 has 4 digits
// 7^5=16807 has 5 digits
// 8^5=32768 has 5 digits
// 9^5=59049 has 5 digits
// 7^6=117649 has 6 digits
// 8^6=262144 has 6 digits
// 9^6=531441 has 6 digits
// 8^7=2097152 has 7 digits
// 9^7=4782969 has 7 digits
// 8^8=16777216 has 8 digits
// 9^8=43046721 has 8 digits
// 8^9=134217728 has 9 digits
// 9^9=387420489 has 9 digits
// 8^10=1073741824 has 10 digits
// 9^10=3486784401 has 10 digits
// 9^11=31381059609 has 11 digits
// 9^12=282429536481 has 12 digits
// 9^13=2541865828329 has 13 digits
// 9^14=22876792454961 has 14 digits
// 9^15=205891132094649 has 15 digits
// 9^16=1853020188851841 has 16 digits
// 9^17=16677181699666569 has 17 digits
// 9^18=150094635296999121 has 18 digits
// 9^19=1350851717672992089 has 19 digits
// 9^20=12157665459056928801 has 20 digits
// 9^21=109418989131512359209 has 21 digits
// Solution: 49
