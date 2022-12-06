use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run() -> Result<u32, String> {
    let file = File::open(Path::new("res/year2022/six/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let mut lines = io::BufReader::new(file).lines();

    let line = lines.next().unwrap().unwrap();
    let chars: Vec<_> = line.chars().collect();

    for i in 0..chars.len()-2 {
        let x = &chars[i..i + 4];
        let x: HashSet<_> = x.iter().collect();
        if x.len() == 4 {
            return Ok((i + 4) as u32);
            break;
        }
    }

    Ok(0)
}