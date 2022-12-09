use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let  data  = fs::read_to_string(file_path)
    .expect("Failed to read input")
    .chars()
    .collect::<Vec<char>>();

    let  val1  = check_window(&data, 4);
    
    println!("Val1 {}", val1);

    let  val2  = check_window(&data, 14);
    
    println!("Val2 {}", val2);
}

fn check_window(data: &Vec<char>, window_size: usize) -> usize {
    data
    .windows(window_size)
    .enumerate()
    .find(|(_i, v)| is_all_unique(v))
    .unwrap().0 + window_size
}

fn is_all_unique(data: &&[char]) -> bool {
    let mut testable_data = data.to_vec();

    testable_data.sort();
    testable_data.dedup();
    if testable_data.len() != data.len() {
        return false;
    }
    true
}