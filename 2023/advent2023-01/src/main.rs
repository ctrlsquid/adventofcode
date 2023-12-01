use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    // Separate each line into a vector
    if let Ok(lines) = read_lines(filepath) {
        // Create a vector to store the numbers, so we can sum them later
        let mut numbers: Vec<i32> = Vec::new();
        // Loop through each line to parse the numbers
        for line in lines {
            if let Ok(string) = line {
                // Loop through each character in the string until a number is found
                let mut first: Option<i32> = None;
                let mut first_string = String::new();
                for (i, c) in string.chars().enumerate() {
                    // If the character is a number, break the loop and store the character
                    if c.is_numeric() {
                        // Convert the character to a number
                        first = Some(c.to_digit(10).unwrap() as i32);
                        break;
                    }
                    // If not, push the character to our string and check if it's in our VALID_NUMBERS array
                    else {
                        first_string.push(c);
                        // Check if any of the VALID_NUMBERS are in the string
                        // Make sure the string is at least 3 characters long, as we don't want to match "a"
                        if first_string.len() >= 3 {
                            // Loop through each VALID_NUMBER, and check if it's in the string
                            let mut found_number: Option<i32> = None;
                            for num in VALID_NUMBERS.iter() {
                                // Check if the string contains the number
                                if first_string.contains(num.0) {
                                    found_number = Some(num.1);
                                    break;
                                }
                            }
                            // If a number was found, break the loop
                            if found_number.is_some() {
                                first = found_number;
                                break;
                            }
                        }
                    }
                }
                // Reverse loop through each character in the string until a number is found
                let mut last: Option<char> = None;
                let mut last_string = String::new();
                for (i, c) in string.chars().rev().enumerate() {
                    // If the character is a number, break the loop
                    if c.is_numeric() {
                        last = Some(c);
                        break;
                    }
                    // If not, push the character to our string and check if it's in our VALID_NUMBERS array
                    // NOTE: The string will be reversed, so we need to reverse it back
                    else {
                        last_string.push(c);
                        // Check if any of the VALID_NUMBERS are in the string
                        // Make sure the string is at least 3 characters long, as we don't want to match "a"
                        if last_string.len() >= 3 {
                            // Loop through each VALID_NUMBER, and check if it's in the string
                            let mut found_number: Option<char> = None;
                            for num in VALID_NUMBERS.iter() {
                                // Check if the string contains the number
                                if last_string.contains(num.0) {
                                    found_number = Some(num.1.to_string().chars().next().unwrap());
                                    break;
                                }
                            }
                            // If a number was found, break the loop
                            if found_number.is_some() {
                                last = found_number;
                                break;
                            }
                        }
                    }
                }
                // Combine the two strings, which should be numbers into a two-digit number
                let number = format!("{}{}", first.unwrap(), last.unwrap()).parse::<i32>().unwrap();
                numbers.push(number);
            }
        }
        // Sum all numbers in the array
        let sum = numbers.iter().sum::<i32>();
        println!("Sum: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}