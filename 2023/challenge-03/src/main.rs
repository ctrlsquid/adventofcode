fn main() {
    let filepath = std::env::args().nth(1).expect("No file provided");
    let mut lines = shared::read_lines(filepath).expect("Error reading file");
    // TODO: Loop through each line and track the current and next line
    lines.enumerate().for_each(|(index, line)| {
        let line = line.unwrap();
        let next_line = lines.next().unwrap().unwrap();
    });
    // TODO: Store the previous line and action on the current line
    // TODO: Check if a number is directly next to a symbol (not counting `.`) on the current line
    // TODO: If not, check if the number is "diagonally" next to a symbol on the current line,
    //      by checking if there's a symbol or number on the previous line within 1 index of the current index
    // TODO: If a number is found next to a symbol, store the number
    // TODO: Summarize the numbers found
}
