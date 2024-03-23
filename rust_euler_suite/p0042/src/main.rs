// https://projecteuler.net/problem=42
// 
// Coded triangle numbers
// 
// Problem 42
// 
// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:
// 
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
// 
// By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.
// 
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?

//use std::env;
use std::fs;

fn word_value(word: String) -> u32
{
    let mut value: u32 = 0;

    for letter in word.chars() {
        value += (letter as u32) - ('A' as u32) + 1;
    }

    return value;
}

fn word_values(words: Vec<String>) -> Vec<u32>
{
    let mut word_values: Vec<u32> = Vec::with_capacity(words.len());

    for word in words {
        word_values.push(word_value(word));
    }

    return word_values;
}

fn split_strings(contents: String) -> Vec<String>
{
    let spl = contents.split(",");
    spl.map(|c| c.trim_matches('"').to_string()).collect()
}

fn generate_triangle_numbers(at_least: u32) -> Vec<u32>
{
    let mut triangle_numbers = Vec::new();
    let mut n = 1;
    let mut last = 1;

    while last < at_least
    {
        last = n*(n+1)/2;
        triangle_numbers.push(last);
        n += 1;
    }

    return triangle_numbers;
}

fn main()
{
    const INPUT: &str = "p042_words.txt";

    let contents = fs::read_to_string(INPUT)
        .expect("Should have been able to read the file");


    let words = split_strings(contents);
    println!("Read {} words", words.len());

    let values = word_values(words);
    let max_word_value = *values.iter().max().unwrap();

    println!("Max word value: {}", max_word_value);

    let triangle_numbers = generate_triangle_numbers(max_word_value);

    let triangle_words = values.iter().filter(|&i| triangle_numbers.contains(i)).count();

    println!("Triangle words: {}", triangle_words);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split_strings(r#""A","ABILITY","ABLE","ABOUT""#.to_string())[0], "A");
    }

    #[test]
    fn test_word_value() {
        assert_eq!(word_value("A".to_string()), 1);
        assert_eq!(word_value("SKY".to_string()), 55);
    }

    #[test]
    fn test_word_values() {
        assert_eq!(word_values(vec!["A".to_string(), "SKY".to_string()]), vec![1, 55]);
    }

    #[test]
    fn test_generate_triangle_numbers() {
        assert_eq!(generate_triangle_numbers(5), vec![1, 3, 6]);
        assert_eq!(generate_triangle_numbers(10), vec![1, 3, 6, 10]);
    }
}

//Read 1786 words
//Max word value: 192
//Triangle words: 162