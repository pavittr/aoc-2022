use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let commands: Vec<Vec<u32>> =
        BufReader::new(File::open(file_path).expect("Cannot open file.txt"))
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

    let mut visibles = 0;

    for x in 1..(commands.len() - 1) {
        for y in 1..(commands[x].len() - 1) {
            println!("({},{}): {}", x, y, commands[y][x]);
            if is_visible(&commands, x, y) {
                visibles += 1;
                println!("Visible: ({},{}): {}", x, y, commands[y][x]);
            }
        }
    }

    println!("Visibles {}", visibles);

    let edges = ((commands.len() - 1) * 2) + (2 * (commands[1].len() - 1));
    println!("Edges {}", edges);

    println!("Totals: {}", (visibles + edges));

    let mut highest_scenic_score = 0;
    for y in 0..(commands.len()) {
        for x in 0..(commands[y].len()) {
            let scenic_score = scenic_score(&commands, x,y);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    println!("Top score : {}", highest_scenic_score);
}

fn scenic_score(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let tree_height = trees[y][x];
    let mut left_score = x;
    for leftx in (0..x).rev() {
        if trees[y][leftx] >= tree_height {
            left_score = x - leftx;
            break;
        }
    }

    let mut right_score = trees[y].len() - x - 1;
    for rightx in (x + 1)..(trees.len()) {
        if trees[y][rightx] >= tree_height {
            right_score = rightx - x;
            break;
        }
    }

    let mut top_score = y;
    for topy in (0..y).rev() {
        if trees[topy][x] >= tree_height {
            top_score = y - topy;
            break;
        }
    }

    let mut bottom_score = trees.len() - y - 1;
    for bottomy in (y+1)..(trees.len()) {
        if trees[bottomy][x] >= tree_height {
            bottom_score = bottomy - y;
            break;
        }
    }

    return left_score * right_score * top_score * bottom_score;
}

fn is_visible(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let tree_height = trees[y][x];
    let mut visible = true;
    for leftx in 0..x {
        println!(
            "Left Comparing ({},{}) to ({},{}). heights {} to {}",
            leftx, y, x, y, trees[y][leftx], tree_height
        );
        if trees[y][leftx] >= tree_height {
            println!(
                "Left is taller ({},{}) to ({},{}). heights {} to {}",
                leftx, y, x, y, trees[y][leftx], tree_height
            );
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;

    for rightx in (x + 1)..(trees.len()) {
        println!(
            "Right Comparing ({},{}) to ({},{}). heights {} to {}",
            rightx, y, x, y, trees[y][rightx], tree_height
        );
        if trees[y][rightx] >= tree_height {
            println!(
                "Right is taller ({},{}) to ({},{}). heights {} to {}",
                rightx, y, x, y, trees[y][rightx], tree_height
            );
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;

    for topy in 0..y {
        if trees[topy][x] >= tree_height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;

    for bottomy in (y + 1)..(trees[y].len()) {
        if trees[bottomy][x] >= tree_height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    return false;
}
