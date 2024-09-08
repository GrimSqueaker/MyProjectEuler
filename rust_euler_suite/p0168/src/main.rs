/*

<p>Consider the number $142857$. We can right-rotate this number by moving the last digit ($7$) to the front of it, giving us $714285$.<br>
It can be verified that $714285 = 5 \times 142857$.<br>
This demonstrates an unusual property of $142857$: it is a divisor of its right-rotation.</p>
<p>Find the last $5$ digits of the sum of all integers $n$, $10 \lt n \lt 10^{100}$, that have this property.</p>

*/

/*

Let d = d_m d_{m-1} ... d_2 d_1
Check if the right rotation d' = d_1 d_m d_{m-1} ... d_2
is divisible by d, i.e. d' = k * d for some integer k.

Let D = d_m d_{m-1} ... d_2. Then
  d  = 10*D + d_1 and
  d' = 10^{m-1}*d_1 + D

Thus, we have
                       d' = k*d
 <=> 10^{m-1}*d_1 + D     = k(10*D + d_1)
 <=> 10^{m-1}*d_1 - k*d_1 = k*10*D - D
 <=> (10^{m-1} - k)*d_1   = (10k - 1)*D
 <=>                    D = (10^{m-1} - k)*d_1 / (10k - 1)

D can be computed for every 0 < m < 100 by iterating over d_1 \in {0, 1, ..., 9} and k \in {1, 2, ..., 9},
If the computed D is an integer then d=10*D+d_1 is one the integers that has the required property.

*/

use lib_number_functions::number_functions::get_right_rotated_biguint;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    let mut accu = 0.to_biguint().unwrap();
    let cutoff = 100000;
    let cutoff_big = cutoff.to_biguint().unwrap();

    // biguint consts
    let ten = 10.to_biguint().unwrap();
    let zero = 0.to_biguint().unwrap();

    for m in 2..=100u32 {
        for d_1 in 0..10 {
            let d_1_big = d_1.to_biguint().unwrap();

            for k in 1..=9 {
                let k_big = k.to_biguint().unwrap();

                // D = (10^{m-1} - k)*d_1 / (10k - 1)
                let num: BigUint = (&ten.pow(m-1) - &k_big) * &d_1_big;
                let denom: BigUint = (10*k - 1).to_biguint().unwrap();

                let rem: BigUint = &num % &denom;
                if &num != &zero && &rem == &zero {
                    let big_d: BigUint = num/denom;

                    let d: BigUint = &big_d*&ten + &d_1_big;
                    let dd: BigUint = &ten.pow(m-1) * &d_1_big + big_d;

                    // check that d is really a divisor of its right rotation
                    // and that the relation is really fulfilled
                    if (get_right_rotated_biguint(&d) == dd) && (dd == d.clone()*&k_big) {
                        println!("Found {} = {} * {}", dd, k, d);

                        accu = accu + d;
                    }
                }
            }
        }
    }

    println!("Accu {}", accu);
    println!("Solution {}", accu % cutoff_big);
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//}


// 1 = 1*1
// 2 = 1*2
// 3 = 1*3
// 4 = 1*4
// 5 = 1*5
// 6 = 1*6
// 7 = 1*7
// 8 = 1*8
// 9 = 1*9
// 11 = 1*11
// 22 = 1*22
// 33 = 1*33
// 44 = 1*44
// 55 = 1*55
// 66 = 1*66
// 77 = 1*77
// 88 = 1*88
// 99 = 1*99
// 111 = 1*111
// 222 = 1*222
// 333 = 1*333
// 444 = 1*444
// 555 = 1*555
// 666 = 1*666
// 777 = 1*777
// 888 = 1*888
// 999 = 1*999
// 1111 = 1*1111
// 2222 = 1*2222
// 3333 = 1*3333
// 4444 = 1*4444
// 5555 = 1*5555
// 6666 = 1*6666
// 7777 = 1*7777
// 8888 = 1*8888
// 9999 = 1*9999
// 11111 = 1*11111
// 22222 = 1*22222
// 33333 = 1*33333
// 44444 = 1*44444
// 55555 = 1*55555
// 66666 = 1*66666
// 77777 = 1*77777
// 88888 = 1*88888
// 99999 = 1*99999
// 111111 = 1*111111
// 512820 = 4*128205
// 714285 = 5*142857
// 615384 = 4*153846
// 717948 = 4*179487
// 820512 = 4*205128
// 222222 = 1*222222
// 923076 = 4*230769
// 333333 = 1*333333
// 444444 = 1*444444
// 555555 = 1*555555
// 666666 = 1*666666
// 777777 = 1*777777
// 888888 = 1*888888
// 999999 = 1*999999
// 1111111 = 1*1111111
// 2222222 = 1*2222222
// 3333333 = 1*3333333
// 4444444 = 1*4444444
// 5555555 = 1*5555555
// 6666666 = 1*6666666
// 7777777 = 1*7777777
// 8888888 = 1*8888888
// 9999999 = 1*9999999
// 11111111 = 1*11111111
// 22222222 = 1*22222222
// 33333333 = 1*33333333
// 44444444 = 1*44444444
// 55555555 = 1*55555555
// 66666666 = 1*66666666
// 77777777 = 1*77777777
// 88888888 = 1*88888888
// 99999999 = 1*99999999
// 111111111 = 1*111111111
// 222222222 = 1*222222222
// 333333333 = 1*333333333
// 444444444 = 1*444444444
// 555555555 = 1*555555555
// 666666666 = 1*666666666
// 777777777 = 1*777777777
// 888888888 = 1*888888888
// 999999999 = 1*999999999
// ...
// Accu 55556698418313450708518822788351788231961404047362254287171550415607326545911081713849885870937959206
// Solution 59206
