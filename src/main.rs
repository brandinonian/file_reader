use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() != 2 {
        panic!("Invalid arguments.");
    }

    let path = args[1].clone();
    let contents = fs::read_to_string(path).expect("Could not read file.");

    println!("{}", contents);
}
