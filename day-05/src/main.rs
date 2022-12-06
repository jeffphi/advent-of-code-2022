use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::VecDeque;

fn main() {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    stacks.push(vec!['N','T','B','S','Q','H','G','R']);
    stacks.push(vec!['J','Z','P','D','F','S','H']);
    stacks.push(vec!['V','H','Z']);
    stacks.push(vec!['H','G','F','J','Z','M']);
    stacks.push(vec!['R','S','M','L','D','C','Z','T']);
    stacks.push(vec!['J','Z','H','V','W','T','M']);
    stacks.push(vec!['Z','L','P','F','T']);
    stacks.push(vec!['S','W','V','Q']);
    stacks.push(vec!['C','N','D','T','M','L','H','W']);
    //println!("{}{}{}{}{}{}{}{}{}", stacks[0][0], stacks[1][0], stacks[2][0], stacks[3][0], stacks[4][0], stacks[5][0], stacks[6][0], stacks[7][0], stacks[8][0]);

    // Oops, wrong order :P
    stacks[0].reverse();
    stacks[1].reverse();
    stacks[2].reverse();
    stacks[3].reverse();
    stacks[4].reverse();
    stacks[5].reverse();
    stacks[6].reverse();
    stacks[7].reverse();
    stacks[8].reverse();

    let mut count = 0;
    let mut from = 0;
    let mut to = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(instruction) = line {
                
                let tokens: Vec<&str> = instruction.split(' ').collect();
                //println!("{} {} {}", tokens[1], tokens[3], tokens[5]);
                count = tokens[1].parse().unwrap();
                from = tokens[3].parse().unwrap();
                to = tokens[5].parse().unwrap();

                // Part 1 Solution
                /*
                while count > 0 {
                    //println!("Count:{} From:{} To:{}", count, from, to);
                    let val = stacks[from-1].pop().unwrap();
                    stacks[to-1].push(val);
                    count -= 1;
                }*/
            
                // Part 2 solution
                let mut temp_stack: Vec<char> = Vec::new();
                count = tokens[1].parse().unwrap();
                while count > 0{
                    let val = stacks[from-1].pop().unwrap();
                    temp_stack.push(val);
                    count -= 1;
                }

                count = temp_stack.len();
                while count > 0{
                    let val = temp_stack.pop().unwrap();
                    stacks[to-1].push(val);
                    count -= 1;
                }
            }

        }
    }

    count = 0;
    while count < 9{
        if stacks[count].len() > 0{
            let val= stacks[count].pop().unwrap();
            println!("{}", val);
        }
        count += 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
