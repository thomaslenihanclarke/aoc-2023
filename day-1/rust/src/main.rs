use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap, HashMap};

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut one_star_res_vec = vec![];
        let mut two_star_res_vec = vec![];
        let search_vec = vec![
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
        ];
        let mut search_map = HashMap::new();

        for search in search_vec {
            match search {
                "one" | "1" => search_map.insert(search, "1"),
                "two" | "2" => search_map.insert(search, "2"),
                "three" | "3" => search_map.insert(search, "3"),
                "four" | "4" => search_map.insert(search, "4"),
                "five" | "5" => search_map.insert(search, "5"),
                "six" | "6" => search_map.insert(search, "6"),
                "seven" | "7" => search_map.insert(search, "7"),
                "eight" | "8" => search_map.insert(search, "8"),
                "nine" | "9" => search_map.insert(search, "9"),
                _ => panic!(),
            };
        }

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(string) = line {

                // One Star
                let num_vec: Vec<char> = string.chars().into_iter().filter(|c| c.is_numeric()).collect();
                let num_count: usize = num_vec.len();

                if num_count > 0 {
                    let one_star_string_res = format!("{}{}", num_vec[0], num_vec[num_count - 1]);
                    let one_star_num_res: i32 = one_star_string_res.parse().unwrap();
                    one_star_res_vec.push(one_star_num_res);
                }

                // Two Star
                let mut found_map = BTreeMap::new();

                for search in &search_map {
                    match string.find(search.0) {
                        Some(num) => found_map.insert(num, search.1),
                        None => continue,
                    };

                    match string.rfind(search.0) {
                        Some(num) => found_map.insert(num, search.1),
                        None => continue,
                    };
                }

                if found_map.len() > 0 {
                    let two_star_string_res = format!("{}{}", found_map.first_key_value().unwrap().1, found_map.last_key_value().unwrap().1);
                    let two_star_num_res: i32 = two_star_string_res.parse().unwrap();
                    two_star_res_vec.push(two_star_num_res);
                }
            }
        }

        let one_star_sum: i32 = one_star_res_vec.iter().sum();
        println!("1 Star total sum is: {one_star_sum}");

        let two_star_sum: i32 = two_star_res_vec.iter().sum();
        println!("2 Star total sum is: {two_star_sum}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}