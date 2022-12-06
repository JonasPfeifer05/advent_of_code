use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let file = File::open(Path::new("res/year2022/three/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let mut lines = io::BufReader::new(file).lines();
    
    let mut score = 0u32;
    for line in lines {
        if let Ok(line) = line {
            let left = &line[..line.len()/2];
            let right = &line[line.len()/2..];

            let right_chars: Vec<_> = right.chars().collect();


            for (i, left_char) in left.chars().enumerate() {
                if right_chars.contains(&left_char) {
                    if left_char.is_uppercase() { score+=26; }
                    let mut char_score = [0];
                    let x: Vec<char> = left_char.to_uppercase().collect();
                    x.get(0).unwrap().encode_utf8(&mut char_score);
                    score += char_score[0] as u32 - 64;
                    break;
                }
            }

        }
    }
    println!("{:?}", score);

    Ok(())
}