use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run() -> Result<[u32; 3], String> {
    let file = File::open(Path::new("res/year2022/one/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let lines = io::BufReader::new(file).lines();

    let mut sums = Vec::new();
    let mut sum = 0;
    for line in lines {
        if let Ok(value) = line {
            if value.trim().len() != 0 {
                let cals: u32 = value.parse().map_err(|_| String::from("Could not parse string"))?;
                sum += cals;
            } else {
                sums.push(sum);
                sum = 0;
            }
        }
    }

    sums.sort();
    sums.reverse();

    Ok([*sums.get(0).unwrap(),*sums.get(1).unwrap(),*sums.get(2).unwrap()])
}