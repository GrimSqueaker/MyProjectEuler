// <p>It is possible to show that the square root of two can be expressed as an infinite continued fraction.</p>
// <p class="center">$\sqrt 2 =1+ \frac 1 {2+ \frac 1 {2 +\frac 1 {2+ \dots}}}$</p>
// <p>By expanding this for the first four iterations, we get:</p>
// <p>$1 + \frac 1 2 = \frac  32 = 1.5$<br>
// $1 + \frac 1 {2 + \frac 1 2} = \frac 7 5 = 1.4$<br>
// $1 + \frac 1 {2 + \frac 1 {2+\frac 1 2}} = \frac {17}{12} = 1.41666 \dots$<br>
// $1 + \frac 1 {2 + \frac 1 {2+\frac 1 {2+\frac 1 2}}} = \frac {41}{29} = 1.41379 \dots$<br></p>
// <p>The next three expansions are $\frac {99}{70}$, $\frac {239}{169}$, and $\frac {577}{408}$, but the eighth expansion, $\frac {1393}{985}$, is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.</p>
// <p>In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?</p>

// use f_1 = 1 + 1/2
//     f_n = 1 + 1/(1 + f_(n-1))

use fraction::BigFraction;

fn main() {
    let mut f_n = BigFraction::new(3u32,2u32);
    let mut count = 0;

    for _i in 2..=1000 {
        f_n = (f_n + 1).recip() + 1;
        let dignum = f_n.numer().unwrap().to_string().len();
        let digdenom = f_n.denom().unwrap().to_string().len();
        
        if dignum > digdenom {
            count += 1;
        }
    }

    println!("More digits in the numerator than the denominator: {}", count)
}

// More digits in the numerator than the denominator: 153
