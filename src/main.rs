use std::io::{BufRead, BufReader};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use array2d::Array2D;
use FSM_TRANSITIONS::*;
#[derive(Copy, Clone)]
enum FSM_TRANSITIONS {
 
    _Reject = 0,
    _Integer = 1,
    _Real = 2,
    _Operator = 3,
    _String = 4,
    _Unknown = 5,
    _Space = 6

}

static rows: Vec<Vec<FSM_TRANSITIONS>> = vec![vec![ _Reject, _Integer, _Real, _Operator, _String, _Unknown, _Space]];

static state_table: array2d::Array2D<FSM_TRANSITIONS> = Array2D::from_rows(&rows);

struct token_type {
    token: String,
    lexeme: u32,
    lexeme_name: String
}

fn main() {
   
    let mut expression = String::new();

    expression = convert_file_to_string( get_file_name() );
    
    lexer(expression);
}

fn get_file_name() -> String {

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

    file.read_to_string(&mut contents);

    contents
}


fn lexer( expression: String ) {
    
    

}

