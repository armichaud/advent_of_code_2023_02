use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>
}
#[derive(Debug)]
struct GameSet {
    red: u32,
    blue: u32,
    green: u32,
}
impl GameSet {
    fn new(red: u32, blue: u32, green: u32) -> GameSet {
        GameSet {
            red,
            blue,
            green,
        }
    }
}


fn get_games(filename: &str) -> Vec<Game> {
    read_to_string(filename).unwrap().lines().map(|line| 
        Game {
            id: line.split(":").next().unwrap().split_whitespace().nth(1).unwrap().trim().parse::<u32>().unwrap(),
            sets: line.split(":").nth(1).unwrap().split(";").map(|set| {
                let mut red: u32 = 0;
                let mut blue: u32 = 0 ;
                let mut green: u32 = 0;
                set.split(",").for_each(|color| {
                    let mut color_value = color.trim().split_whitespace();
                    let n = color_value.next().unwrap().parse::<u32>().unwrap();
                    match color_value.next().unwrap().trim() {
                        "red" => red = n,
                        "blue" => blue = n,
                        "green" => green = n,
                        _ => panic!("Invalid color")
                    }
                });
                GameSet::new(red, blue, green)
            }).collect(),
        }
    ).collect()
}

fn sum_of_possible_game_ids(filename: &str) -> u32 {
    let games = get_games(filename);
    let comparison_set = GameSet::new(12, 14, 13);
    let mut sum = 0;
    for game in games {
        let mut possible = true;
        for set in game.sets {
            if set.red > comparison_set.red || set.blue > comparison_set.blue || set.green > comparison_set.green {
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
            min_blue = min_blue.max(set.blue);
            min_green = min_green.max(set.green);
        }
        sum += (min_red * min_blue * min_green) as u32;
    }
    sum
}

fn main() {
    assert_eq!(sum_of_possible_game_ids("example.txt"), 8);
    assert_eq!(sum_of_possible_game_ids("input.txt"), 3035);
    assert_eq!(sum_of_power_of_minimum_sets("example.txt"), 2286);
    assert_eq!(sum_of_power_of_minimum_sets("input.txt"), 66027);
}
