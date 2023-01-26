use std::{fmt::Debug, collections::HashMap};
use std::fs::{File};
use std::io::Write;
use std::path::Path;

use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    /// Can change path to be vector
    path: Vec<std::path::PathBuf>,
}

fn main() {

    let args = Cli::parse();
    println!("{:?}",args);
    
    let mut dict: HashMap<&str,(i32,Vec<usize>)> = HashMap::new();

    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut file_vec: Vec<String> = Vec::new();
    for file in &args.path {
        file_vec.push(std::fs::read_to_string(file).expect("could not read file"));
    }

    for file in &file_vec {
        for (i,line) in file.lines().enumerate() {
            if line.contains(&args.pattern) {
                //Default count and also needs line
                let count = dict.entry(&args.pattern).or_insert((0,vec![i]));
                count.0 += 1;
                count.1.push(i);
            }
        }
    }

    if !Path::new("./matches.txt").exists() {
        let mut f = File::create("matches.txt").expect("Error creating file");
    }
    for (k,v) in dict.iter(){
        println!("{0} occured {1} times on lines {2:?}.",k,v.0,v.1);
        write!("./matches.txt", format!("{0} occured {1} times on lines {2:?}.",k,v.0,v.1)).expect("Error writing to file");

    }

}

#[test]
fn number_on_main() {
    let args = Cli {pattern:String::from("std"),path:String::from("src/main.rs")};

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for (i,line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            // assert_eq!() println!("Line {0} contains: {1}", i, line);
        }
    }

}