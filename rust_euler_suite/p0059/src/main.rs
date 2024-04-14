//
// <p>Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.</p>
// <p>A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given value, taken from a secret key. The advantage with the XOR function is that using the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.</p>
// <p>For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random bytes. The user would keep the encrypted message and the encryption key in different locations, and without both "halves", it is impossible to decrypt the message.</p>
// <p>Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key. If the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message. The balance for this method is using a sufficiently long password key for security, but short enough to be memorable.</p>
// <p>Your task has been made easy, as the encryption key consists of three lower case characters. Using <a href="resources/documents/0059_cipher.txt">0059_cipher.txt</a> (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes, and the knowledge that the plain text must contain common English words, decrypt the message and find the sum of the ASCII values in the original text.</p>
// 

use std::io::{BufRead, BufReader};
use std::fs::File;

type Key = [char;3];

fn main() {
    const INPUT: &str = "0059_cipher.txt";

    let buf = read_cipher_line(INPUT);
    let cipher_text = convert_csv_numbers_to_string(&buf);
    let key = find_key(&cipher_text);
    let plain_text = decipher_text(&cipher_text, key);
    let sum = sum_ascii_values(&plain_text);

    println!("Plain text");
    println!("{}", plain_text);
    println!("Sum of ASCII chars: {}", sum);
}

fn read_cipher_line(filename: &str) -> String {
    let mut reader = BufReader::new(File::open(filename)
        .expect("Cannot open file.txt"));

    let mut buf: String = String::new();
    reader.read_line(&mut buf)
        .expect("Should have a line.");

    buf
}

fn convert_csv_numbers_to_string(line: &str) -> String {
    let mut text: String = String::new();

    line.split(',')
        .map(|x| x.parse::<u8>().unwrap() as char)
        .for_each(|c| text.push(c));

    text
}

fn find_key(cipher_text: &str) -> Key {
    let mut histogram: [[u32;256];3] = [[0;256];3];

    cipher_text.chars()
        .enumerate()
        .for_each(|(n,c)| histogram[n%3][c as usize] += 1);

    let mut guess_key: Key = [' ';3];
    for i in 0..3 {
        let found_most = histogram[i]
            .iter()
            .enumerate()
            .max_by_key(|(_idx, &val)| val);

        // assume that space is the most common character in the plain text
        guess_key[i] = ((' ' as u8) ^ (found_most.unwrap().0 as u8)) as char;
    }

    guess_key
}

fn decipher_text(cipher_text: &str, key: Key) -> String {
    cipher_text.chars()
        .enumerate()
        .map(|(n, c)| ((c as u8) ^ (key[n%key.len()] as u8)) as char)
        .collect()
}

fn sum_ascii_values(text: &str) -> u64 {
    text.chars()
        .fold(0, |acc, x| acc+(x as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_csv_numbers_to_char_vector() {
        assert_eq!(convert_csv_numbers_to_string("65,42,107"), "A*k".to_string());
    }

    #[test]
    fn test_try_decipher() {
        assert_eq!(decipher_text(&"A*k".to_string(), [0 as char,0 as char,0 as char]), "A*k".to_string());
        assert_eq!(decipher_text(&"A*k".to_string(), [2 as char,0 as char,1 as char]), "C*j".to_string());
    }

    #[test]
    fn test_sum_ascii_values() {
        assert_eq!(sum_ascii_values(&"A*k".to_string()), 65+42+107);
    }
}

// Plain text
// An extract taken from the introduction of one of Euler's most celebrated papers, "De summis serierum reciprocarum" [On the sums of series of reciprocals]: I have recently found, quite unexpectedly, an elegant expression for the entire sum of this series 1 + 1/4 + 1/9 + 1/16 + etc., which depends on the quadrature of the circle, so that if the true sum of this series is obtained, from it at once the quadrature of the circle follows. Namely, I have found that the sum of this series is a sixth part of the square of the perimeter of the circle whose diameter is 1; or by putting the sum of this series equal to s, it has the ratio sqrt(6) multiplied by s to 1 of the perimeter to the diameter. I will soon show that the sum of this series to be approximately 1.644934066842264364; and from multiplying this number by six, and then taking the square root, the number 3.141592653589793238 is indeed produced, which expresses the perimeter of a circle whose diameter is 1. Following again the same steps by which I had arrived at this sum, I have discovered that the sum of the series 1 + 1/16 + 1/81 + 1/256 + 1/625 + etc. also depends on the quadrature of the circle. Namely, the sum of this multiplied by 90 gives the biquadrate (fourth power) of the circumference of the perimeter of a circle whose diameter is 1. And by similar reasoning I have likewise been able to determine the sums of the subsequent series in which the exponents are even numbers.
// Sum of ASCII chars: 129448
