use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let file = File::open(Path::new("res/year2022/three/part1.txt")).map_err(|_| String::from("Could not read file!"))?;
    let mut lines = io::BufReader::new(file).lines();

    let mut score = 0u32;

    loop {
        if let Some(Ok(one)) = lines.next() {
            if let Some(Ok(two)) = lines.next() {
                if let Some(Ok(three)) = lines.next() {
                    let two: Vec<_> = two.chars().collect();
                    let three: Vec<_> = three.chars().collect();

                    for one_char in one.chars() {
                        if two.contains(&one_char) && three.contains(&one_char) {
                            if one_char.is_uppercase() { score+=26; }
                            let mut char_score = [0];
                            let x: Vec<char> = one_char.to_uppercase().collect();
                            x.get(0).unwrap().encode_utf8(&mut char_score);
                            score += char_score[0] as u32 - 64;
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    println!("{:?}", score);

    Ok(())
}