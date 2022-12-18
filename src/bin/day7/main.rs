use std::env;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let commands : Vec<String>  = BufReader::new(File::open(file_path)
    .expect("Cannot open file.txt"))
    .lines()
    .map(|line| line.unwrap())
    .collect();

    let mut currentpath: Vec<String> = vec![ "".to_string(); 1];
    let mut paths: HashMap<String, u32>  = HashMap::new();
    let mut path_sizes : HashMap<String, u32> = HashMap::new();
    path_sizes.insert("/".to_string(), 0);

    commands.iter().for_each(|cmd| {
        println!("Processing {}", cmd);
        println!("Currently in {}", currentpath.join("/"));

        if cmd.starts_with("$ cd") {
            // We;re changing the path
            println!("Need to change the path");
            let cmd_path = cmd.split_whitespace().nth(2).unwrap();
            match cmd_path {
                "/" => currentpath.truncate(1),
                ".." => _ = currentpath.pop(),
                _ => currentpath.push(cmd_path.to_string()),

            }

        } else if cmd.starts_with("$ ls") {
            // We're about to look at data
            println!("Listign folders");
        } else  if cmd.starts_with("dir") {
            // We're about to look at data
            println!("Lookign at a folder on the listing");
        } else {
            // We're listign content
            println!("This is content");
            let mut file_info = cmd.split_whitespace();
            let file_size = file_info.next().unwrap();
            let file_path = file_info.next().unwrap();
            // Add this to the path sections
            let mut path_parts = currentpath.to_owned();
            while path_parts.len() > 1 {
                let cur_path = path_parts.join("/");
                let curVal: u32 = *path_sizes.get(&cur_path).unwrap_or(&0);

                println!("Potential Path: {}. Size {}", cur_path, curVal);
                path_sizes.insert(path_parts.join("/"), curVal + file_size.parse::<u32>().unwrap());
                path_parts.pop();
            }

            path_sizes.insert("/".to_string(), path_sizes.get("/").unwrap() + file_size.parse::<u32>().unwrap());
            paths.insert(currentpath.join("/")+ "/" + file_path, file_size.parse::<u32>().unwrap());
        }
    }
    );

    let total_size =  path_sizes.iter().filter(|(k,v)| v < &&(100000 as u32)).fold(0, |acc, (k,v)| acc + v);
    println!("Sum {}", total_size);


    let used_space = path_sizes.get("/").unwrap();
    let free_space = 70000000 - used_space;

    let need_to_find = 30000000 - free_space;

    println!("Free space = {}, need {}", free_space, need_to_find);

    let (smallest_folder_to_delete,smallest_folder_size) =  path_sizes.iter().filter(|(k,v)| v > &&(need_to_find) ).min_by(|(k1,v1),(k2,v2)| v1.cmp(v2)).unwrap();
    println!("Can delete {}, size {}", smallest_folder_to_delete, smallest_folder_size);


}
