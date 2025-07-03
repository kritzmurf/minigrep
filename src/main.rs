use std::{env, fs};

fn main() {
    //get CLI args
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for: {}", query);
    println!("In filepath: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Expected to be able to read the file");

    println!("file contents:\n{contents}");

}
