use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run() -> Result<char, String> {
    let file = File::open(Path::new("res/year2022/five/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let mut lines = io::BufReader::new(file).lines();

    let mut instructions = false;

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in lines {
        if let Ok(line) = line {
            if line.len() == 0 {
                instructions = true;
                for i in 0..stacks.len() {
                    stacks.get_mut(i).unwrap().reverse();
                }
                continue;
            }

            if instructions {
                let mut elements: Vec<_> = line.split(" ").collect();

                let amount: u32 = elements.get(1).unwrap().parse().unwrap();
                let from: u32 = elements.get(3).unwrap().parse().unwrap();
                let to: u32 = elements.get(5).unwrap().parse().unwrap();

                let from = stacks.get_mut(from as usize-1).unwrap();
                let mut tmp = Vec::new();
                for i in 0..amount {
                    tmp.push(from.pop().unwrap());
                }
                let to = stacks.get_mut(to as usize-1).unwrap();
                for i in 0..amount {
                    to.push(tmp.pop().unwrap());
                }

            } else {
                let mut chars = line.chars();
                for i in 0..stacks.len() {
                    chars.next();
                    if let Some(character) = chars.next() {
                        if character != ' ' && character.is_alphabetic() {
                            stacks.get_mut(i).unwrap().push(character);
                        }
                    }
                    chars.next();
                    chars.next();
                }
            }
        }
    }
    stacks.iter().for_each(|x| println!("{}", x.get(x.len()-1).unwrap()));

    Ok(' ')
}