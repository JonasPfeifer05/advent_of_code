use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::path::Path;

pub fn run() -> Result<u32, String> {
    let file = File::open(Path::new("res/year2022/two/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let lines = io::BufReader::new(file).lines();

    let mut mapping: HashMap<&str, u32> = HashMap::new();
    mapping.insert("A", 0);
    mapping.insert("X", 0);
    mapping.insert("B", 1);
    mapping.insert("Y", 1);
    mapping.insert("C", 2);
    mapping.insert("Z", 2);

    let mut points = 0u32;

    for line in lines {
        if let Ok(line) = line {
            let enemy = &line.trim()[..1];
            let player = &line.trim()[2..];

            points += evaluate_points(enemy, player, &mapping);

            println!("{}", enemy);
            println!("{}", player);
        }
    }

    Ok(points)
}

pub fn evaluate_points(enemy: &str, player: &str, mapping: &HashMap<&str, u32>) -> u32 {
    let p1 = *mapping.get(enemy).unwrap();


    let p2 = match player {
        "Z" => {
            (p1 + 1) % 3+1 + 6
        }
        "Y" => {
            p1+1 + 3
        }
        "X" => {
            let vec2: Vec<u32> = vec![p1, (p1 + 1)%3];
            for i in 0..3 {
                if !vec2.contains(&i) {
                    return i+1
                }
            }
            0
        }
        _ => {0}
    };

    p2
}