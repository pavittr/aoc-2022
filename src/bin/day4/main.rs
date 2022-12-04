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
    .map(|l| l.unwrap())
    .filter(|line| has_full_overlap(line))
    .count().try_into().unwrap();
    println!("Score {}", score);

    let p2_score : u32  = BufReader::new(File::open(file_path)
    .expect("Cannot open file.txt"))
    .lines()
    .map(|l| l.unwrap())
    .filter(|line| has_partial_overlap(line))
    .count().try_into().unwrap();
    println!("P2 Score {}", p2_score);

}

fn parse( line: &String) -> (i32, i32, i32, i32) {
    let (lhs,rhs) = line.split_once(",").unwrap();
    let (lhs_start_str, lhs_end_str) = lhs.split_once("-").unwrap();
    let (rhs_start_str, rhs_end_str) = rhs.split_once("-").unwrap();
    let lhs_start = lhs_start_str.parse::<i32>().unwrap();
    let lhs_end = lhs_end_str.parse::<i32>().unwrap();
    let rhs_start = rhs_start_str.parse::<i32>().unwrap();
    let rhs_end = rhs_end_str.parse::<i32>().unwrap();
(lhs_start, lhs_end, rhs_start, rhs_end) 
}

fn has_full_overlap(line: &String) -> bool {

    let (lhs_start, lhs_end, rhs_start, rhs_end) = parse(line);
    
    if (lhs_start >= rhs_start && lhs_end <= rhs_end) 
    || (lhs_start <= rhs_start && lhs_end >= rhs_end) {
        return true;
    }
    false
}

fn has_partial_overlap(line: &String) -> bool {
    let (lhs_start, lhs_end, rhs_start, rhs_end) = parse(line);
    
    if lhs_end < rhs_start || rhs_end < lhs_start {
        return false;
    }
    true

}
