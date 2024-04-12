// <p>Starting with $1$ and spiralling anticlockwise in the following way, a square spiral with side length $7$ is formed.</p>
// <p class="center monospace"><span class="red"><b>37</b></span> 36 35 34 33 32 <span class="red"><b>31</b></span><br>
// 38 <span class="red"><b>17</b></span> 16 15 14 <span class="red"><b>13</b></span> 30<br>
// 39 18 <span class="red"> <b>5</b></span>  4 <span class="red"> <b>3</b></span> 12 29<br>
// 40 19  6  1  2 11 28<br>
// 41 20 <span class="red"> <b>7</b></span>  8  9 10 27<br>
// 42 21 22 23 24 25 26<br><span class="red"><b>43</b></span> 44 45 46 47 48 49</p>
// <p>It is interesting to note that the odd squares lie along the bottom right diagonal, but what is more interesting is that $8$ out of the $13$ numbers lying along both diagonals are prime; that is, a ratio of $8/13 \approx 62\%$.</p>
// <p>If one complete new layer is wrapped around the spiral above, a square spiral with side length $9$ will be formed. If this process is continued, what is the side length of the square spiral for which the ratio of primes along both diagonals first falls below $10\%$?</p>


use primes::{Sieve, PrimeSet};

// step:         1,  2,  3,  4,  5  n
// side length:  1,  3,  5,  7,  9  l(n) = 2n-1
// ends:         1,  9, 25, 49, 81  e(n) = l(n)*l(n)
// diagonal values:  d(n) = ( e(n-1)+l(n)-1, e(n-1)+2(l(n)-1), e(n-1)+3(l(n)-1), e(n-1)+4(l(n)-1) )

fn main() {
    let side_length = first_side_length_with_ratio_lower_than(1.0/10.0);

    println!("Side length with first prime ratio < 10%: {}", side_length);
}

fn first_side_length_with_ratio_lower_than(target_ratio: f32) -> u64 {
    let mut prim = 0u64;
    let mut diag = 1u64;

    let mut pset = Sieve::new();
    pset.find(100000);

    for step in 2.. {
        diag += 4;
        prim += get_diagonal_values_for_step(step).iter()
            .fold(0, |acc, n| acc + if pset.is_prime(*n as u64) {1} else {0});

        let ratio = (prim as f32) / (diag as f32);
        if ratio < target_ratio {
            return get_side_length(step);
        }

        println!("Step {}: length {}, primes {}, diagonals {}, end {}, ratio {}", 
            step, get_side_length(step), prim, diag, get_spiral_end_for_step(step), ratio);
    }

    0
}

fn get_side_length(step: u64) -> u64 {
    2*step-1
}

fn get_spiral_end_for_step(step: u64) -> u64 {
    get_side_length(step)*get_side_length(step)
}

fn get_diagonal_values_for_step(step: u64) -> [u64; 4] {
    let e = get_spiral_end_for_step(step-1);
    let l = get_side_length(step) - 1;
    [e+l, e+2*l, e+3*l, e+4*l]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_side_length() {
        assert_eq!(get_side_length(1), 1);
        assert_eq!(get_side_length(5), 9);
    }

    #[test]
    fn test_get_spiral_end_for_step() {
        assert_eq!(get_spiral_end_for_step(1), 1);
        assert_eq!(get_spiral_end_for_step(2), 9);
        assert_eq!(get_spiral_end_for_step(3), 25);
        assert_eq!(get_spiral_end_for_step(4), 49);
        assert_eq!(get_spiral_end_for_step(5), 81);
    }

    #[test]
    fn test_get_diagonal_values_for_step() {
        assert_eq!(get_diagonal_values_for_step(2), [3, 5, 7, 9]);
        assert_eq!(get_diagonal_values_for_step(3), [13, 17, 21, 25]);
        assert_eq!(get_diagonal_values_for_step(4), [31, 37, 43, 49]);
    }

}

// Side length with first prime ratio < 10%: 26241
