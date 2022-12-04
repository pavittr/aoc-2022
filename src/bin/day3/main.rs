use std::env;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let score : u32  = BufReader::new(File::open(file_path)
    .expect("Cannot open file.txt"))
    .lines()
    .map(|line| determine_p1_score(&line.unwrap()))
    .sum();
    println!("Score {}", score);

    let p2_score: u32  = BufReader::new(File::open(file_path)
    .expect("Cannot open file.txt"))
    .lines()
    .map(|l| l.unwrap())
    .collect::<Vec<String>>().chunks(3).map(|group| p2(group)).sum();
    println!("P2 Score {}", p2_score);
}

fn p2(group: &[String]) -> u32 {
    let mut result = String::from(group[0].clone());
    result.retain(|c| group[1].contains(c));
    result.retain(|c| group[2].contains(c));
    parse( result.chars().next().unwrap())
}

fn parse(c : char) -> u32 {
    if c.is_ascii_uppercase() {
        return c as u32 - 65 + 27;
    }
    return c as u32 - 97 + 1;
}


fn determine_p1_score(hand: &str) -> u32 {
    let midpoint = hand.len() / 2;
    let (lhs,rhs) = hand.split_at(midpoint);
    let mut result = String::from(lhs);
    result.retain(|c| rhs.contains(c));
    parse(result.chars().next().unwrap())
}
