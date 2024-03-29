// Permuted Multiples
// 
// Problem 52
// 
// It can be seen that the number, 125874, and its double, 251748,
// contain exactly the same digits, but in a different order.
// 
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.

fn main() {
    let mut found = 0u64;
    let mut max_multiple_same_digits = 0;

    for number in 1..=1000000000 {
        let multiple_same_digits = get_max_multiple_count_with_same_digits(number);

        if multiple_same_digits > max_multiple_same_digits {
            print!("Current max number of same digits: {} -> ", multiple_same_digits);
            print!("{}", number);
            for i in 2..=multiple_same_digits {
                print!(", {}", i*number);
            }
            println!("");
            max_multiple_same_digits = multiple_same_digits;
            found = number;
        }

        if max_multiple_same_digits == 6 {
            break;
        }
    }

    println!("Found max number of same digits: {} -> {}", max_multiple_same_digits, found);

}

fn get_max_multiple_count_with_same_digits(number: u64) -> u64 {
    let mut check = number;
    let digits = get_digits(number);

    let mut multiple = 1;

    while multiple < 6 {
        check += number;

        if get_digits(check) == digits {
            multiple += 1;
        }
        else {
            break
        }
    }

    return multiple;
}

fn get_digits(mut number: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];

    while number > 0 {
        let remainder = number % 10;
        result.push(remainder);
        number /= 10;
    }

    result.sort();
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(123), vec![1,2,3]);
        assert_eq!(get_digits(286), vec![2,6,8]);
        assert_eq!(get_digits(11586), vec![1,1,5,6,8]);
        assert_eq!(get_digits(11006), vec![0,0,1,1,6]);
    }

    #[test]
    fn test_get_max_multiple_count_with_same_digits()  {
        assert_eq!(get_max_multiple_count_with_same_digits(125874), 2);
    }
}

// Current max number of same digits: 1 -> 1
// Current max number of same digits: 2 -> 125874, 251748
// Current max number of same digits: 6 -> 142857, 285714, 428571, 571428, 714285, 857142
// Found max number of same digits: 6 -> 142857
