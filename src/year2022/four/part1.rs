use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Range;
use std::path::Path;

pub fn run() -> Result<u32, String> {
    let file = File::open(Path::new("res/year2022/four/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let mut lines = io::BufReader::new(file).lines();

    let mut count = 0u32;

    for line in lines {
        if let Ok(line) = line {
            let ranges: Vec<_> = line.trim().split(",").collect();

            let left: Vec<_> = ranges.get(0).unwrap().split("-").collect();
            let right: Vec<_> = ranges.get(1).unwrap().split("-").collect();

            let left: (u32, u32) = (left.get(0).unwrap().parse().unwrap(), left.get(1).unwrap().parse().unwrap());
            let right: (u32, u32) = (right.get(0).unwrap().parse().unwrap(), right.get(1).unwrap().parse().unwrap());

            if left.0 > right.0 {
                if left.1 <= right.1 {
                    count += 1;
                }
            } else if right.0 > left.0 {
                if right.1 <= left.1 {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
    }

    Ok(count)
}