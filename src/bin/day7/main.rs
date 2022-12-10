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
            
            paths.insert(currentpath.join("/")+ "/" + file_path, file_size.parse::<u32>().unwrap());
        }

    }
        
    );


    paths.iter().for_each(|(k,v)| println!("{}={}", k, v));


}
