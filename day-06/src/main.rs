use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let data_vec: Vec<char> = data.chars().collect();
                let mut index = 4;
                let mut tracker = HashMap::new();
                while index <= data_vec.len(){
                    tracker.insert(data_vec[index-1], 0);
                    tracker.insert(data_vec[index-2], 0);
                    tracker.insert(data_vec[index-3], 0);
                    tracker.insert(data_vec[index-4], 0);

                    if tracker.len() == 4 {
                        break;
                    }
                    tracker.clear();
                    index += 1;
                }
                println!("Index: {}", index);

                //Part 2
                index += 14;

                while index <= data_vec.len(){
                    tracker.insert(data_vec[index-1], 0);
                    tracker.insert(data_vec[index-2], 0);
                    tracker.insert(data_vec[index-3], 0);
                    tracker.insert(data_vec[index-4], 0);
                    tracker.insert(data_vec[index-5], 0);
                    tracker.insert(data_vec[index-6], 0);
                    tracker.insert(data_vec[index-7], 0);
                    tracker.insert(data_vec[index-8], 0);
                    tracker.insert(data_vec[index-9], 0);
                    tracker.insert(data_vec[index-10], 0);
                    tracker.insert(data_vec[index-11], 0);
                    tracker.insert(data_vec[index-12], 0);
                    tracker.insert(data_vec[index-13], 0);
                    tracker.insert(data_vec[index-14], 0);

                    if tracker.len() == 14 {
                        break;
                    }
                    tracker.clear();
                    index += 1;
                }
                println!("Index 2: {}", index);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
