fn main() {
    let filepath = std::env::args().nth(1).expect("No file provided");
    if let Ok(lines) = shared::read_lines(filepath) {
        // Parse all games out of the input by line
        let games: Vec<Game> = lines.map(|line| Game::from(line.unwrap())).collect();
        // Filter out any games that have a larger value than the maximum allowed
        let max_red_allowed = 12;
        let max_green_allowed = 13;
        let max_blue_allowed = 14;
        // PART 1 - Get all games that don't have a larger value than the maximum allowed and sum their IDs
        let sum_ids_filtered_max: i32 = games
            .iter()
            .clone()
            .filter(|game| {
                game.min_red <= max_red_allowed &&
                game.min_green <= max_green_allowed &&
                game.min_blue <= max_blue_allowed
            })
            .map(|game| game.id)
            .sum();
        println!("Sum of valid game IDs: {}", sum_ids_filtered_max);
        // PART 2 - Get all games again, but multiply the minimum values instead of summing the IDs
        let product_min_values: i32 = games
            .iter()
            .map(|game| game.min_red * game.min_green * game.min_blue)
            .sum();
        println!("Product of minimum values: {}", product_min_values);
    } else {
        println!("Error reading file");
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    min_red: i32,
    min_green: i32,
    min_blue: i32,
}

impl From<String> for Game {
    fn from(string: String) -> Self {
        // Example expected input: Game 64: 5 red, 2 green; 5 green, 2 red, 2 blue; 3 red, 3 blue, 1 green; 3 blue, 3 green, 3 red; 1 green, 3 red
        // The input is expected to a game, an ID, and then a list of colors separated by semicolons
        // Each color is separated by commas, and may or may not contain all three colors
        // First, we split the string be a colon to get the ID and the colors
        let mut split = string.split(": ");
        // Get the ID
        let id = split.next().unwrap().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        // Store the colors in a vector
        let all_colors = split.next().unwrap();
        // Split the colors into groups by semicolons
        let color_groups = all_colors.split("; ");
        // Iterate over each color group and split the colors by commas
        // Store the largest value of the colors
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for group in color_groups {
            let colors = group.split(", ");
            // Iterate over each color and store the largest value
            for color in colors {
                let mut split = color.split(' ');
                let number = split.next().unwrap().parse::<i32>().unwrap();
                let color = split.next().unwrap();
                match color {
                    "red" => {
                        if number > max_red {
                            max_red = number;
                        }
                    }
                    "green" => {
                        if number > max_green {
                            max_green = number;
                        }
                    }
                    "blue" => {
                        if number > max_blue {
                            max_blue = number;
                        }
                    }
                    _ => panic!("Invalid color: {}", color),
                }
            }
        }

        Game {
            id,
            min_red: max_red,
            min_green: max_green,
            min_blue: max_blue,
        }
    }
}