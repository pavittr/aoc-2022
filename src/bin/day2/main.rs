use std::env;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let score : i32  = BufReader::new(File::open(file_path)
    .expect("Cannot open file.txt"))
    .lines()
    .map(|line| determine_p1_score(&line.unwrap()))
    .sum();
    println!("Score {}", score);

    let actual_score : i32  = BufReader::new(File::open(file_path)
    .expect("Cannot open file.txt"))
    .lines()
    .map(|line| determine_p2_score(&line.unwrap()))
    .sum();
    println!("Actual Score {}", actual_score);

}


fn determine_p1_score(hand: &str) -> i32 {
    // Options:
    // A X -> Rock Rock Draw 1 + 3
    // A Y -> Rock Paper Win 2 + 6
    // A Z -> Rock Scissors Lose 3 + 0
    // B X -> Paper Rock Lose 1 + 0
    // B Y -> Paper Paper Draw 2 + 3
    // B Z -> Paper Scissors Win 3 + 6
    // C X -> Scissors Rock Win 1 + 6
    // C Y -> Scissors Paper Lose 2 + 0
    // C Z -> Scissors Scissors Draw 3 + 3
    return match hand {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
         &_ => todo!()
    }
}

fn determine_p2_score(hand: &str) -> i32 {
    // Scores:
    // Win: 6
    // Draw: 3
    // Lose 0
    // Rock: 1
    // Paper: 2
    // Scissors: 3
    // Needed outcomes:
    // X: Lose
    // Y: Draw
    // Z: Win
    //
    // Their hand:
    // A: Rock
    // B: Paper
    // C: Scissors
    //
    // Options:
    // A X -> 0 + 3 
    // A Y -> 3 + 1
    // A Z -> 6 + 2
    // B X -> 0 + 1
    // B Y -> 3 + 2
    // B Z -> 6 + 3
    // C X -> 0 + 2
    // C Y -> 3 + 3
    // C Z -> 6 + 1
    return match hand {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
         &_ => todo!()
    }
}
