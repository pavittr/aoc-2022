use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let max =  fs::read_to_string(file_path)
        .expect("Failed to read input")
        .split("\n\n")
        .map(|s| s.to_string())
        .map(|block| block
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|l| l.parse::<i32>().unwrap())
            .sum::<i32>()).max().unwrap(); 
    println!("Max '{}'", max);
    let mut sorted =  fs::read_to_string(file_path)
        .expect("Failed to read input")
        .split("\n\n")
        .map(|s| s.to_string())
        .map(|block| block
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|l| l.parse::<i32>().unwrap())
            .sum::<i32>()).collect::<Vec<i32>>();

            sorted.sort();
            sorted.reverse();
    println!("Top 3: '{}'", sorted[0] + sorted[1] + sorted[2]);
}
