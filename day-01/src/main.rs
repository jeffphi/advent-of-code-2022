use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut biggest = 0;
    let mut sum = 0;
    let mut vec: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                if calories.chars().count() == 0 {
                    println!("{}","New line...");
                    if sum > biggest {
                        biggest = sum;
                    }
                    vec.push(sum);
                    sum = 0;
                    continue;
                }
                else {
                    println!("{}", calories);
                    let tempNum = calories.parse::<i32>().unwrap();
                    sum += tempNum;
                }
            }
        }
    }

    vec.sort();
    println!("Biggest: {}", biggest);
    println!("{}", vec[0]);
    println!("{}", vec[1]);
    println!("{}", vec[2]);
    let top_three = vec[vec.len()-1] + vec[vec.len()-2] + vec[vec.len()-3];
    println!("Top three: {}", top_three);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
