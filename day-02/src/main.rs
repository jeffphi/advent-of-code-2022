use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("A X"), 4);
    scores.insert(String::from("A Y"), 8);
    scores.insert(String::from("A Z"), 3);
    scores.insert(String::from("B X"), 1);
    scores.insert(String::from("B Y"), 5);
    scores.insert(String::from("B Z"), 9);
    scores.insert(String::from("C X"), 7);
    scores.insert(String::from("C Y"), 2);
    scores.insert(String::from("C Z"), 6);

    let mut part_2_map: HashMap<String, String> = HashMap::new();
    part_2_map.insert(String::from("A X"),  String::from("A Z"));
    part_2_map.insert(String::from("A Y"), String::from("A X"));
    part_2_map.insert(String::from("A Z"), String::from("A Y"));
    part_2_map.insert(String::from("B X"), String::from("B X"));
    part_2_map.insert(String::from("B Y"), String::from("B Y"));
    part_2_map.insert(String::from("B Z"), String::from("B Z"));
    part_2_map.insert(String::from("C X"), String::from("C Y"));
    part_2_map.insert(String::from("C Y"), String::from("C Z"));
    part_2_map.insert(String::from("C Z"), String::from("C X"));

    let mut score = 0;
    let mut part_2_score = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game) = line {
                // println!("Looking for:{}", game);
                score += scores[&game];

                // Part 2
                part_2_score += scores[&part_2_map[&game]];
            }
        }
    }
    println!("Total score: {}", score);
    println!("Part 2 score: {}", part_2_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
