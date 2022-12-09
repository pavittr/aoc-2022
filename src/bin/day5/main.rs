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


    p1(&sections);
    p2(&sections);
}


fn p1(sections: &Vec<String>) {
    let shape_lines = sections[0]
    .rsplit("\n")
    .map(|l| l.to_string())
    .filter(|l| l.starts_with("[") || l.starts_with("  "))
    .collect::<Vec<String>>();

    let mut stacks = vec![Stack{ crates: vec![' '; 0] }; (shape_lines[0].len() +1 )/4];

    for line in shape_lines.iter() {
        let mut stack = 0;
        line.chars().collect::<Vec<char>>().chunks(4).for_each(|chunk| { if chunk [1] != ' ' {stacks[stack].crates.push(chunk[1]) } stack+=1});
    }

    sections[1]
    .split("\n")
    .map(|l| l.to_string().split_whitespace().filter_map(|l| l.parse::<usize>().ok()).collect::<Vec<usize>>())
    .for_each(|l| update_stacks(l, &mut stacks));
    
     println!("P1: {}", stacks.iter().map(|stack| stack.crates.last().unwrap()).collect::<String>());
}

fn p2(sections: &Vec<String>) {
    let shape_lines = sections[0]
    .rsplit("\n")
    .map(|l| l.to_string())
    .filter(|l| l.starts_with("[") || l.starts_with("  "))
    .collect::<Vec<String>>();

    let mut stacks = vec![Stack{ crates: vec![' '; 0] }; (shape_lines[0].len() +1 )/4];

    for line in shape_lines.iter() {
        let mut stack = 0;
        line.chars().collect::<Vec<char>>().chunks(4).for_each(|chunk| { if chunk [1] != ' ' {stacks[stack].crates.push(chunk[1]) } stack+=1});
    }

    sections[1]
    .split("\n")
    .map(|l| l.to_string().split_whitespace().filter_map(|l| l.parse::<usize>().ok()).collect::<Vec<usize>>())
    .for_each(|l| bulk_update_stacks(l, &mut stacks));

     println!("P2: {}", stacks.iter().map(|stack| stack.crates.last().unwrap()).collect::<String>());
}

fn bulk_update_stacks(instruction: Vec<usize>, stacks : &mut Vec<Stack>) {
    let batch_size = instruction[0];
    let from: usize = instruction[1] - 1;
    let to: usize = instruction[2] - 1;

    // Take would be easier here but its in nightly
    let mut elems = vec![' '; 0];
    for _i in 0..batch_size {
        elems.push(stacks[from].crates.pop().unwrap());
        
    }
    elems.reverse();
    elems.iter().for_each(|elem|  stacks[to].crates.push(*elem));
}


fn update_stacks(instruction: Vec<usize>, stacks : &mut Vec<Stack>) {
    let loop_size = instruction[0];
    let from: usize = instruction[1] - 1;
    let to: usize = instruction[2] - 1;

    for _i in 0..loop_size {
        let elem = stacks[from].crates.pop().unwrap();
        stacks[to].crates.push(elem);
    }
}

#[derive(Clone)]
pub struct Stack {

    crates: Vec<char>,
}