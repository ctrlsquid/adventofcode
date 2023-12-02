fn main() {
    let filepath = std::env::args().nth(1).expect("No file provided");
    // Store the largest value of cube colors: red, green, and blue
    if let Ok(lines) = shared::read_lines(filepath) {
        // Check if the cube number for each relative color is larger than the current largest
        // If the any value is larger than the maximum value, skip the entire line
    }
}