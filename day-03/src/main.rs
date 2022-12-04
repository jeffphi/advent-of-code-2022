use substring::Substring;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority: HashMap<char, i32> = HashMap::new();

    for (i, c) in letters.chars().enumerate() {
        priority.insert(c, (i+1).try_into().unwrap());
    }

    let mut sum = 0;
    let mut sum_part_2 = 0;

    let mut sack_count = 1;
    let mut sack_1 = String::new();
    let mut sack_2 = String::new();
    let mut sack_3 = String::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(rucksack) = line {
                let compartment_1 = rucksack.substring(0,rucksack.len()/2);
                let compartment_2 = rucksack.substring(rucksack.len()/2,rucksack.len());
                //println!("Ruchsack: {}", rucksack);
                //println!("compartment 1: {}", compartment_1);
                //println!("compartment 2: {}",  compartment_2);
                for c in compartment_1.chars(){
                    if compartment_2.contains(c) {
                        //println!("Match: {}, {}",c, priority[&c]);
                        sum += priority[&c];
                        //println!("Cur sum: {}", sum);
                        break;
                    }
                }

                // Part 2
                match sack_count%3 {
                    1 => sack_1 = rucksack, 
                    2 => sack_2 = rucksack,
                    0 => { sack_3 = rucksack;
                        for c in sack_1.chars(){
                            if sack_2.contains(c) && sack_3.contains(c){
                                sum_part_2 += priority[&c];
                                break;
                            }
                        }
                    },
                    _ => println!("No match!?"),
                }
            }
            sack_count += 1;
        }
    }
    
    println!("Priority sum: {}", sum);
    println!("Priority part 2: {}", sum_part_2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
