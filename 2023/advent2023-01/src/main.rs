use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filepath = std::env::args().nth(1).expect("No file provided");
    // TODO: Get first number in string
    // TODO: Get last number in string
    // TODO: Combine into two-digit number
    // TODO: Add number to array
    // TODO: Repeat for all numbers in string
    // TODO: Sum all numbers in array
    // Separate each line into a vector
    if let Ok(lines) = read_lines(filepath) {
        let mut numbers: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(string) = line {
                // Loop through each character in the string until a number is found
                let mut first: Option<char> = None;
                let mut last: Option<char> = None;
                for (i, c) in string.chars().enumerate() {
                    if c.is_numeric() {
                        // Convert the character to a number
                        first = Some(c);
                        break;
                    }
                }
                // Reverse loop through each character in the string until a number is found
                for (i, c) in string.chars().rev().enumerate() {
                    if c.is_numeric() {
                        last = Some(c);
                        break;
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