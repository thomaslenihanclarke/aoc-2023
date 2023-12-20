use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    if let Ok(lines) = read_lines("../input.txt") {
        let mut one_star_res = 0;
        let mut two_star_res = 0;

        for line in lines {
            if let Ok(string) = line {
                println!("{string}");

                let mut round_poss = true;
                let game_string = string.split(": ").next().unwrap();
                let game_num = game_string.split("Game ").filter_map(|x| x.parse::<i32>().ok()).next().unwrap();
                let game_full = format!("Game {}: ", game_num);
                let rounds = string.split(game_full.as_str()).filter(|&x| !x.is_empty()).next().unwrap();
                let rounds_split = rounds.split("; ").filter(|&x| !x.is_empty());

                let mut maxes = HashMap::from([
                    ("red", 0),
                    ("green", 0),
                    ("blue", 0),
                ]);

                for round in rounds_split {
                    let turns = round.split(", ").filter(|&x| !x.is_empty());

                    for turn in turns {
                        let mut turn_parts = turn.split(" ").filter(|&x| !x.is_empty());
                        let amount = turn_parts.next().unwrap().parse::<i32>().ok().unwrap();
                        let colour = turn_parts.next().unwrap();

                        if round_poss {
                            round_poss = amount <= limits[&colour];
                        }

                        if amount > maxes[&colour] {
                            maxes.insert(colour, amount);
                        }
                    }
                }

                if round_poss {
                    one_star_res = one_star_res + game_num;
                }

                let round_power = maxes.values().cloned().reduce(|a, b| a * b).unwrap();
                two_star_res = two_star_res + round_power;
            }
        }

        println!("one star total: {one_star_res}");
        println!("two star total: {two_star_res}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
