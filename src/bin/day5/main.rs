use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let sections: Vec<String> =  fs::read_to_string(file_path)
    .expect("Failed to read input")
    .split("\n\n")
    .map(|s| s.to_string())
    .collect();

    let mut shape_lines = sections[0]
    .split("\n")
    .map(|l| l.to_string())
    .collect::<Vec<String>>();

    let legend = shape_lines.pop().unwrap();
    let data = legend.split_ascii_whitespace().last().expect("No number I guess?");
    println!("Data {}", data);
    // let array_size:usize = data.parse::<usize>().unwrap();
    // println!("Size: {}", array_size);
    // println!("Block SHape: '{}'", shape_lines[0]);

    
    // let poles : Vec<Vec<String>> = Vec::new();
    // A line will have length 
    //  1
    // [A]
    //    3
    //     [B]
    //        7
    //         [C]
    //            11
    // Length is:
    // 1: 3
    // 2: 7
    // 3: 11
    // n: 4n-1
    // SO we can use this to figure out what the length of each line is. 

    // Take a line, look at its length, add 1 and divide by 4 to get the number of characters
    // take every 2nd, 6th, 10th etc char up tot eh limit and push them onto a vector and return it. 
    // Then fill the pole witht he vector


    
    println!("Section1:\n{}", sections[0]);
}
