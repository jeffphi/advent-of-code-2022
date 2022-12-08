use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

enum  FileSystemObj {
    Dir {
        name: String, 
        subdirs: HashMap<String, Box<FileSystemObj>>,
        files: HashMap<String, i32>, 
        parent: Box<FileSystemObj>},
        Null,
}

fn main() {

    let mut root = FileSystemObj::Dir{
        name: String::from("/"),
        subdirs: HashMap::new(),
        files: HashMap::new(),
        parent: Box::new(FileSystemObj::Null),
    };

    // let mut path: Vec<&FileSystemObj> = Vec::new();
    // path.push(&root);
    let mut cur_dir = &mut root;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(cmd) = line {
                let tokens: Vec<&str> = cmd.split(' ').collect();

                //Look at first letter
                match tokens[0] {
                    // Commands
                    "$" => {
                        //println!{"Command: {}", cmd};
                        //Only expecting cd or ls...
                        match tokens[1] {
                            "cd" => {
                                //println!("cd to {}", tokens[2]);
                                match tokens[2] {
                                    ".." => {
                                        if let FileSystemObj::Dir{name:_, subdirs:_, files:_,  parent} = cur_dir {
                                            cur_dir = parent;
                                        }
                                    },
                                    _ => {
                                        // cd to a new dir. Let's assume that it's been seen before
                                        // (leveraging knowledge about the input file)

                                        if let FileSystemObj::Dir{name:_, subdirs, files:_, parent:_} = cur_dir {
                                            cur_dir = &mut subdirs.get_mut(tokens[3]).unwrap();
                                        }
                                    }
                                }
                            },
                            "ls" => {/*Can skip ls, data follows...*/},
                            _ => {println!("Unexpected cmd: {}", tokens[1]);}
                        }
                    },
                    // Directory or File data 
                    _ => {
                        println!{"Data: {}", cmd}
                        match tokens[0] {
                            "dir" => {
                            },
                            _ => { //Must be a file, let's add it to the cur_dir's HashMap of files
                                if let FileSystemObj::Dir{name:_, subdirs:_, ref mut files, parent:_} = cur_dir {
                                    files.insert(String::from(tokens[1]),tokens[0].parse().unwrap());
                                }
                            }
                        }
                    }
                }        
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

