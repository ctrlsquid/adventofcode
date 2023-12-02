use std::io::{BufRead};
use shared::read_lines;

const VALID_NUMBERS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let filepath = std::env::args().nth(1).expect("No file provided");
    if let Ok(lines) = read_lines(filepath) {
        // Create a vector of numbers, containing the calculated numbers for each line
        let mut numbers: Vec<i32> = Vec::new();
        // Loop through each line to parse the numbers
        for line in lines {
            if let Ok(string) = line {
                // First, find the first number in the string
                // This is done by looping through the string forwards
                let first = get_first_number(&string, false).expect("No first number found");
                // Then, find the last number in the string
                let last = get_first_number(&string, true).expect("No last number found");
                // Combine the two strings, which should be numbers into a two-digit number
                let number = format!("{}{}", first, last).parse::<i32>().unwrap();
                numbers.push(number);
            }
        }
        // Sum all numbers in the array
        let sum = numbers.iter().sum::<i32>();
        println!("Sum: {}", sum);
    }
}

// Gets the first number in a string
fn get_first_number(string: &str, to_reverse: bool) -> Option<i32> {
    // A partial string for storing each character if it isn't a number
    let mut partial_string = String::new();
    // The string to loop through, either the original string or the reversed string
    let checked_string = if to_reverse {
        string.chars().rev().collect::<String>()
    } else {
        string.to_string()
    };
    // Loop through each character in the string
    // If it's a number, return it
    // If it isn't, store it in the partial string and check that partial string for a number
    for (_i, c) in checked_string.chars().enumerate() {
        // First, check if the singular character is a number
        // If it is, break the loop and store the character
        if let Some(num) = get_number_from_string(&c.to_string()) {
            return Some(num);
        }
        // If not, then store the character in our string and check the entire string
        partial_string.push(c);
        // If the checked string is reversed, then we need to reverse the partial string
        let to_check = if to_reverse {
            partial_string.chars().rev().collect::<String>()
        } else {
            partial_string.to_string()
        };
        if let Some(num) = get_number_from_string(&to_check) {
            return Some(num);
        }
    }
    // If no number is found, return None
    None
}

// Checks a given string for a number, either by character or by "word", and returns it if found
fn get_number_from_string(string: &str) -> Option<i32> {
    // If the string's length is 1, check if that character is a number
    if string.len() == 1 {
        // If the character is a number, convert it to a number and return it
        if string.chars().nth(0).unwrap().is_numeric() {
            return Some(string.chars().nth(0).unwrap().to_digit(10).unwrap() as i32);
        }
    }
    // If the string isn't at least 3 characters long, return None
    if string.len() < 3 {
        return None;
    }
    // Lastly, if the string is at least 3 characters long, check if it contains any of the VALID_NUMBERS
    for num in VALID_NUMBERS.iter() {
        // Check if the string contains the number
        if string.contains(num.0) {
            return Some(num.1)
        }
    }
    // If no number is found, return None
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_first_number() {
        let input = "onesevenseven5fourlrkkqtfkrmdlsmd";
        assert_eq!(super::get_first_number(input, false), Some(1));
        assert_eq!(super::get_first_number(input, true), Some(4));
    }
    #[test]
    fn get_number_from_string() {
        let input = "sixfobfzvdthre";
        assert_eq!(super::get_number_from_string(input), Some(6));
    }
}