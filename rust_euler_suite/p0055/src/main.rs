// <p>If we take $47$, reverse and add, $47 + 74 = 121$, which is palindromic.</p>
// <p>Not all numbers produce palindromes so quickly. For example,</p>
// \begin{align}
// 349 + 943 &amp;= 1292\\
// 1292 + 2921 &amp;= 4213\\
// 4213 + 3124 &amp;= 7337
// \end{align}
// <p>That is, $349$ took three iterations to arrive at a palindrome.</p>
// <p>Although no one has proved it yet, it is thought that some numbers, like $196$, never produce a palindrome. A number that never forms a palindrome through the reverse and add process is called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome. In fact, $10677$ is the first number to be shown to require over fifty iterations before producing a palindrome: $4668731596684224866951378664$ ($53$ iterations, $28$-digits).</p>
// <p>Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is $4994$.</p>
// <p>How many Lychrel numbers are there below ten-thousand?</p>
// <p class="smaller">NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.</p>


const MAX_STEPS: u64 = 50;

type Digits = Vec<u8>;


fn main() {
    //
}

fn count_lychrel_up_to(n: u32) -> u32 {
    0
}

fn is_lychrel_number(number: u128) -> bool {
    false
}

fn split_into_digits(number: u128) -> Digits {
    vec![]
}

fn check_palindrome(digits: &Digits) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lychrel_number() {
        assert_eq!(is_lychrel_number(1), false);
        assert_eq!(is_lychrel_number(47), false);
        assert_eq!(is_lychrel_number(349), false);
        assert_eq!(is_lychrel_number(196), true);
    }

    #[test]
    fn test_split_into_digits() {
        assert_eq!(split_into_digits(1), vec![1]);
        assert_eq!(split_into_digits(123), vec![1,2,3]);
        assert_eq!(split_into_digits(1234567890987654321), vec![1,2,3,4,5,6,7,8,9,0,9,8,7,6,5,4,3,2,1]);
    }

    #[test]
    fn test_check_palindrome() {
        assert_eq!(check_palindrome(&vec![1]), true);
        assert_eq!(check_palindrome(&vec![1,1]), true);
        assert_eq!(check_palindrome(&vec![1,2,1]), true);
        assert_eq!(check_palindrome(&vec![9,6,1,2,1,6,9]), true);
        assert_eq!(check_palindrome(&vec![1,2,3,4,5,6,7,8,7,6,5,4,3,2,1]), true);
        assert_eq!(check_palindrome(&vec![1,2]), false);
        assert_eq!(check_palindrome(&vec![1,2,3,4,5,6,7,7,8,6,5,4,3,2,1]), false);
    }
}
