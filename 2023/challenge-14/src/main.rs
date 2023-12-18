fn main() {
    // First read in our input, which should look similar to this:
    // O..#.#..O
    // .O#.#O#.#
    // .#O..#OO.
    let lines: Vec<String> = shared::read_lines("./input.txt")
        .expect("Failed to read input").map(
            |line| line.expect("Failed to read line")
        ).collect();
    // TODO: For each string in array, check if its a
    //  - O for a round rock
    //  - # for a square rock
    //  - . for an empty space
    // First, check if the current index is a round rock, if so move forward with moving it
    // Depending on the direction, check if the directional space is empty, if so move there (replace . with O)
    // A space is not considered empty unless it's a .
    // After all O's have been moved, check the distance from each O in a line to the last index and sum for the total distance (load)
    // At the end, sum all loads to get the total load
}

enum Rock {
    RoundRock,
    SquareRock,
    EmptySpace
}

// Gets the rock from the line at the given index
fn get_rock_from_line(index: usize, line: &str) -> Rock {
    let character = line.chars().nth(index).unwrap();
    match character {
        'O' => Rock::RoundRock,
        '#' => Rock::SquareRock,
        '.' => Rock::EmptySpace,
        _ => panic!("Unexpected character: {}", character)
    }
}

// Calculates the load for a given line by counting the round rocks and multiplying by the multiplier
fn calculate_load_for_line(line: &str, multiplier: usize) -> usize {
    // Count all the round rocks in the line
    let mut round_rocks = 0;
    for character in line.chars() {
        if character == 'O' {
            round_rocks += 1;
        }
    }
    // Multiply the round rocks by the multiplier
    round_rocks * multiplier
}