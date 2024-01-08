use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>
}
#[derive(Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}


fn get_games(filename: &str) -> Vec<Game> {
    read_to_string(filename).unwrap().lines().map(|line| 
        Game {
            id: line.split(":").next().unwrap().split_whitespace().nth(1).unwrap().trim().parse::<u32>().unwrap(),
            sets: line.split(":").nth(1).unwrap().split(";").map(|set| {
                let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);
                set.split(",").for_each(|color| {
                    let mut color_value = color.trim().split_whitespace();
                    let n = color_value.next().unwrap().parse::<u32>().unwrap();
                    match color_value.next().unwrap().trim() {
                        "red" => red = n,
                        "green" => green = n,
                        "blue" => blue = n,
                        _ => panic!("Invalid color")
                    }
                });
                GameSet { red, green, blue }
            }).collect(),
        }
    ).collect()
}

fn sum_of_possible_game_ids(filename: &str) -> u32 {
    let games = get_games(filename);
    let comparison_set = GameSet { red: 12, green: 13, blue: 14 };
    let mut sum = 0;
    for game in games {
        let mut possible = true;
        for set in game.sets {
            if set.red > comparison_set.red || set.green > comparison_set.green || set.blue > comparison_set.blue  {
                possible = false;
                break;
            }
        }
        if possible {
            sum += game.id;
        }
    }
    sum
}

fn sum_of_power_of_minimum_sets(filename: &str) -> u32 {
    let games = get_games(filename);
    let mut sum = 0;
    for game in games {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for set in game.sets {
            min_red = min_red.max(set.red);
            min_green = min_green.max(set.green);
            min_blue = min_blue.max(set.blue);
        }
        sum += (min_red * min_green * min_blue) as u32;
    }
    sum
}

fn main() {
    println!("{}", sum_of_possible_game_ids("example.txt"));
    println!("{}", sum_of_possible_game_ids("input.txt"));
    println!("{}", sum_of_power_of_minimum_sets("example.txt"));
    println!("{}", sum_of_power_of_minimum_sets("input.txt"));
}
