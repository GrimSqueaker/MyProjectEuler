//Take the number 192 and multiply it by each of 1, 2, and 3:
//
//    192 × 1 = 192
//    192 × 2 = 384
//    192 × 3 = 576
//
//By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)
//
//The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
//
//What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?


fn is_1_9_pandigital(number: u64) -> bool {
    if (number < 100000000) || (number > 999999999) {
        return false;
    }

    let mut n = number;
    let mut present = [false; 10];

    while n > 0 {
        let digit = (n % 10) as usize;
        n /= 10;

        if present[digit] == true {
            return false;
        }
        else {
            present[digit] = true;
        }
    }

    if present[0] {
        return false;
    }

    for i in 1..10 {
        if !present[i] {
            return false;
        }
    }

    return true;
}

fn truncated_concatenated_product(d: u64, n: u64) -> u64 {
    let mut result = 0;
    let clamp = 1000000000u64;

    for s in 1..n+1 {
        let prod = d*s;
        result *= 10u64.pow(1 + prod.ilog10());
        result += prod;
        if result >= clamp {
            return clamp;
        }
    }

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital() {
        assert_eq!(is_1_9_pandigital(1), false);
        assert_eq!(is_1_9_pandigital(123456799), false);
        assert_eq!(is_1_9_pandigital(1234567890), false);
        assert_eq!(is_1_9_pandigital(123456789), true);
        assert_eq!(is_1_9_pandigital(193456782), true);
    }

    #[test]
    fn test_concatenated() {
        assert_eq!(truncated_concatenated_product(1, 1), 1);
        assert_eq!(truncated_concatenated_product(2, 1), 2);
        assert_eq!(truncated_concatenated_product(2, 2), 24);
        assert_eq!(truncated_concatenated_product(1, 5), 12345);
        assert_eq!(truncated_concatenated_product(192, 3), 192384576);
        assert_eq!(truncated_concatenated_product(9, 5), 918273645);
        assert_eq!(truncated_concatenated_product(192, 4), 1000000000);
    }
}


fn main() {
    let mut largest = 0u64;
    for num in 1..100000 {
        for mult in 2..10 {
            let prod = truncated_concatenated_product(num, mult);
            if is_1_9_pandigital(prod) {
                println!("The concatenated product of {} and 1..{} is {} which is 1-9-pandigital", num, mult, prod);
                if largest < prod {
                    largest = prod;
                }
            }
        }
    }

    println!("The largest 1-9-pandigital is {}", largest);
}

//The concatenated product of 1 and 1..9 is 123456789 which is 1-9-pandigital
//The concatenated product of 9 and 1..5 is 918273645 which is 1-9-pandigital
//The concatenated product of 192 and 1..3 is 192384576 which is 1-9-pandigital
//The concatenated product of 219 and 1..3 is 219438657 which is 1-9-pandigital
//The concatenated product of 273 and 1..3 is 273546819 which is 1-9-pandigital
//The concatenated product of 327 and 1..3 is 327654981 which is 1-9-pandigital
//The concatenated product of 6729 and 1..2 is 672913458 which is 1-9-pandigital
//The concatenated product of 6792 and 1..2 is 679213584 which is 1-9-pandigital
//The concatenated product of 6927 and 1..2 is 692713854 which is 1-9-pandigital
//The concatenated product of 7269 and 1..2 is 726914538 which is 1-9-pandigital
//The concatenated product of 7293 and 1..2 is 729314586 which is 1-9-pandigital
//The concatenated product of 7329 and 1..2 is 732914658 which is 1-9-pandigital
//The concatenated product of 7692 and 1..2 is 769215384 which is 1-9-pandigital
//The concatenated product of 7923 and 1..2 is 792315846 which is 1-9-pandigital
//The concatenated product of 7932 and 1..2 is 793215864 which is 1-9-pandigital
//The concatenated product of 9267 and 1..2 is 926718534 which is 1-9-pandigital
//The concatenated product of 9273 and 1..2 is 927318546 which is 1-9-pandigital
//The concatenated product of 9327 and 1..2 is 932718654 which is 1-9-pandigital
//The largest 1-9-pandigital is 932718654
