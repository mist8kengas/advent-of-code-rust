#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part1(list: Vec<String>) -> u32 {
    let mut sum = 0;

    // explicitly define the legal possible game
    const LEGAL_GAME: Game = Game {
        red: 12,
        green: 13,
        blue: 14,
    };

    'item_loop: for item in list {
        // get respective value regions
        let part: Vec<&str> = item.split(": ").collect();

        // get game id as int
        let game_id = part[0]
            .matches(char::is_numeric)
            .collect::<Vec<&str>>()
            .concat()
            .parse::<u32>()
            .unwrap();

        // get respective sets and put into an iter
        let game: Vec<&str> = part[1].split("; ").collect();

        // parse set in each game
        for i in game {
            let set: Vec<&str> = i.split(", ").collect();

            for pair_str in set {
                let pair: Vec<&str> = pair_str.split(" ").clone().collect();
                let color = pair[1];
                let nums = pair[0].parse::<u32>().unwrap_or(0);

                let mut game = Game {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                match color {
                    "red" => game.red = nums,
                    "green" => game.green = nums,
                    "blue" => game.blue = nums,
                    _ => (),
                }

                // check if the Game struct is within legal bounds
                // if game is illegal, skip the current loop
                if LEGAL_GAME.red < game.red
                    || LEGAL_GAME.green < game.green
                    || LEGAL_GAME.blue < game.blue
                {
                    continue 'item_loop;
                }
            }
        }

        sum += game_id
    }

    return sum;
}

pub fn part2(list: Vec<String>) -> u32 {
    let mut sum = 0;

    for item in list {
        // get respective value regions
        let part: Vec<&str> = item.split(": ").collect();

        // get respective sets and put into an iter
        let game_sets: Vec<&str> = part[1].split("; ").collect();

        let mut min_game_set = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        // parse set in each game
        for i in game_sets {
            let set: Vec<&str> = i.split(", ").collect();

            for pair_str in set {
                let pair: Vec<&str> = pair_str.split(" ").clone().collect();
                let color = pair[1];
                let nums = pair[0].parse::<u32>().unwrap_or(0);

                match color {
                    "red" => {
                        if min_game_set.red < nums {
                            min_game_set.red = nums
                        }
                    }
                    "green" => {
                        if min_game_set.green < nums {
                            min_game_set.green = nums
                        }
                    }
                    "blue" => {
                        if min_game_set.blue < nums {
                            min_game_set.blue = nums
                        }
                    }
                    _ => (),
                }
            }
        }

        // calculate the powers of each game set
        // and add the powers to the sum
        let pows_of_set = min_game_set.red * min_game_set.green * min_game_set.blue;
        sum += pows_of_set
    }

    return sum;
}
