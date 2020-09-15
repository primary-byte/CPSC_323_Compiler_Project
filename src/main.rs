use std::io::{BufReader};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::env;   //for command line arguments, eg filename.txt

use FsmTransitions::*;


#[derive(Copy, Clone, Debug)]
enum FsmTransitions {
 
    _Reject = 0,
    _Integer = 1,
    _Real = 2,
    _Operator = 3,
    _String = 4,
    _Unknown = 5,
    _Space = 6

}

const STATE_TABLE: &[&[FsmTransitions]]  = &[
    &[_Reject, _Integer, _Real, _Operator, _String, _Unknown, _Space],  //row 0
    &[_Reject, _Integer, _Real, _Operator, _String, _Unknown, _Space]   //row 1
];



struct TokenType {
    token: String,
    lexeme: u32,
    lexeme_name: String
}

fn main() {
   
    println!("State Table: ");

    //rows
    for row in STATE_TABLE.iter(){        
        //columns
        for value in row.iter(){
            print!(" {:?}", value);
        }
        print!("\n"); //new line
    }


    let expression = convert_file_to_string( get_file_name() );
    
    lexer(expression);
}

fn get_file_name() -> String {

    //check for command line arguments
    let args: Vec<String> = env::args().collect();
    
    //if we have command line argument then use that instead of asking the user
    if args.len() > 1 {
        return args[1].to_string();
    }

    //otherwise ask for input
    let mut file_name = String::new();

    println!("Please enter the name of the file: ");
    
    io::stdin()
        .read_line( &mut file_name )
        .expect("Failed to read file name");

    file_name
}

fn convert_file_to_string( file_name: String) -> String {
   
    let mut contents = String::new();

    let mut file = BufReader::new( File::open( file_name.trim() )
                                   .expect("Can't open file!"));

    file.read_to_string(&mut contents)
    .expect("Error reading file!");

    contents
}


fn lexer( expression: String ) {
    
    //do stuff for lexing
    print!("{}",expression);

}

