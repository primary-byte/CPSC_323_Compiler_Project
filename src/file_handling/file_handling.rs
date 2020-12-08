#[path = "../lexer/lexer.rs"]
pub mod lexer;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader; //for command line arguments, eg filename.txt
use std::io::Write as IoWrite;

pub fn get_file_name(count: &mut i32) -> String {
    //check for command line arguments
    let args: Vec<String> = env::args().collect();
    //if we have command line argument then use that instead of asking the user.print!
    //  first returned will be the input file, second will be the output file

    if args.len() > 1 && *count == 0 {
        *count = 1;
        return args[1].to_string();
    } else if args.len() > 2 && *count == 1 {
        return args[2].to_string();
    }

    //otherwise ask for input
    let mut file_name = String::new();

    if (count as &i32) == &0 {
        println!("Please enter the path to the input file: ");
    } else {
        println!("Please enter the path to the output file: ");
    }

    *count = 1;

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read file name");

    file_name
}

//read a file to a string and return the string
pub fn convert_file_to_string(file_name: String) -> String {
    let mut contents = String::new();

    let mut file = BufReader::new(File::open(file_name.trim()).expect("Can't open file!"));

    file.read_to_string(&mut contents)
        .expect("Error reading file!");

    contents.push_str(" $");
    contents
}

//write a given vector of tokens to a given file
pub fn write_to_file(file_name: String, token: Vec<lexer::TokenType>) {
    //let mut line: String;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_name.trim())
        .unwrap();

    //iterate over tokens
    for tok in token.iter() {
        let line = format!("{}         {}", &tok.lexeme_name, &tok.token);
        if let Err(e) = writeln!(file, "{:?}", line) {
            eprintln!("Could not write to file: {}", e);
        }
    }
}
