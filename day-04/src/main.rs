use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut count = 0;
    let mut part_2_count = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(assignment) = line {
                //println!("Assignment: {}", assignment);
                let elves: Vec<&str> = assignment.split(',').collect();
                
                let elf_1: Vec<&str> = elves[0].split("-").collect();
                let elf_1_low: i32 = elf_1[0].parse().unwrap();
                let elf_1_high: i32 = elf_1[1].parse().unwrap();
                //println!("Elf 1: {}-{}",elf_1_low, elf_1_high);

                let elf_2: Vec<&str> = elves[1].split("-").collect();
                let elf_2_low: i32 = elf_2[0].parse().unwrap();
                let elf_2_high: i32 = elf_2[1].parse().unwrap();
                //println!("Elf 2: {}-{}",elf_2_low, elf_2_high);

                if (elf_1_low >= elf_2_low && elf_1_high <= elf_2_high) 
                    || (elf_2_low >= elf_1_low && elf_2_high <= elf_1_high){ 
                    //println!("Incrementing!");
                    count += 1;
                    //println!("Count: {}", count);
                }
                if ((elf_1_low >= elf_2_low ) && (elf_1_low <= elf_2_high)) ||
                    ((elf_1_high >= elf_2_low) && (elf_1_high <= elf_2_high)) ||
                    ((elf_2_low >= elf_1_low) && (elf_2_low <= elf_1_high))||
                    ((elf_2_high >= elf_1_low) && (elf_2_high <= elf_1_high)){
                        part_2_count += 1;
                }
            }
        }

        println!("Count: {}", count);
        println!("Part 2 Count: {}", part_2_count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
