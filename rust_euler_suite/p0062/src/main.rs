
// <p>The cube, $41063625$ ($345^3$), can be permuted to produce two other cubes: $56623104$ ($384^3$) and $66430125$ ($405^3$). In fact, $41063625$ is the smallest cube which has exactly three permutations of its digits which are also cube.</p>
// <p>Find the smallest cube for which exactly five permutations of its digits are cube.</p>


fn main() {
    let mut cubes: Vec<u64> = vec![0,1];
    let mut current: usize = 1;

    loop {
        let cube = cubes[current];
        let cube_largest_perm = get_largest_permutation_of_digits(cube);
        extend_cubes_to(cube_largest_perm+1, &mut cubes);

        let mut permutations = 1;

        let mut check = current+1;
        while cubes[check] <= cube_largest_perm {
            if is_permutation(cube, cubes[check]) {
                permutations += 1;
            }
            check += 1;
        }

        if permutations == 5 {
            break;
        }
        if permutations == 4 {
            println!("4 Permutations: {}", cube);
        }

        current += 1;
    }

    println!("Smallest cube with 5 permutations: {}", cubes[current]);
}

fn extend_cubes_to(num: u64, cubes: &mut Vec<u64>) {
    while *cubes.last().unwrap() < num {
        let next = cubes.len() as u64;
        cubes.push(next*next*next);
    }
}

fn get_largest_permutation_of_digits(mut num: u64) -> u64 {
    let mut digits: Vec<u64> = vec![];

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits.sort();

    let mut result = 0;

    digits.iter()
        .rev()
        .for_each(|d| {
            result *= 10;
            result += *d;
        });

    result
}

fn is_permutation(n1: u64, n2: u64) -> bool {
    get_largest_permutation_of_digits(n1) == get_largest_permutation_of_digits(n2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_cubes_to() {
        let mut cubes: Vec<u64> = vec![0,1];
        extend_cubes_to(27, &mut cubes);
        assert_eq!(cubes, vec![0,1,8,27]);
        extend_cubes_to(50, &mut cubes);
        assert_eq!(cubes, vec![0,1,8,27,64]);
    }

    #[test]
    fn test_get_largest_permutation_of_digits() {
        assert_eq!(get_largest_permutation_of_digits(1234), 4321);
        assert_eq!(get_largest_permutation_of_digits(12340), 43210);
        assert_eq!(get_largest_permutation_of_digits(10), 10);
    }

    #[test]
    fn test_is_permutation() {
        assert_eq!(is_permutation(1234, 1234), true);
        assert_eq!(is_permutation(1234, 11234), false);
        assert_eq!(is_permutation(1234, 12340), false);
        assert_eq!(is_permutation(981234, 123489), true);
    }
}

// 4 Permutations: 1006012008
// 4 Permutations: 10190085632
// 4 Permutations: 17840960397
// 4 Permutations: 105756712489
// 4 Permutations: 109489762304
// Smallest cube with 5 permutations: 127035954683
