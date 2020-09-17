use std::io::{BufReader};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::env;   //for command line arguments, eg filename.txt

use FsmTransitions::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FsmTransitions {
 
    _Reject,
    _Integer,
    _Real,
    _Operator,
    _String,
    _Unknown,
    _Space,

}

const STATE_TABLE: &[&[FsmTransitions]]  = &[
    &[ _Reject,   _Integer, _Real,    _Operator, _String,  _Unknown, _Space ], //Default
    &[ _Integer,  _Integer, _Real,    _Reject,   _Reject,  _Reject,  _Reject], //State 1
    &[ _Real,     _Real,    _Unknown, _Reject,   _Reject,  _Reject,  _Reject], //State 2
    &[ _Operator, _Reject,  _Reject,  _Reject,   _String,  _Reject,  _Reject], //State 3
    &[ _String,   _String,  _Reject,  _String,   _String,  _Reject,  _Reject], //State 4
    &[ _Unknown,  _Unknown, _Unknown, _Unknown,  _Unknown, _Unknown, _Reject], //State 5
    &[ _Space,    _Reject,  _Reject,  _Reject,   _Reject,  _Reject,  _Reject], //State 6
];


#[derive(Clone)]
struct TokenType {
    token: String,
    lexeme: FsmTransitions,
    lexeme_name: String,
}

impl Default for TokenType {
    fn default () -> TokenType {
        TokenType{token: "".to_string(), lexeme: _Reject, lexeme_name: "".to_string()}
    }
}

/*fn build_token(token: String, lexeme: u32, lexeme_name: String) -> TokenType {
   TokenType{
       token,
       lexeme,
       lexeme_name,
   }
}*/

fn main() {
   /*
    println!("State Table: ");

    //rows
    for row in STATE_TABLE.iter(){        
        //columns
        for value in row.iter(){
            print!(" {:?}", value);
        }
        print!("\n"); //new line
    }*/



    let expression = convert_file_to_string( get_file_name() );
    let mut token: Vec<TokenType> = lexer( &expression );

    for tok in token.iter() {
        println!("{}        {}", tok.lexeme_name, tok.token);
    }
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


fn lexer( expression: &String ) -> Vec<TokenType> {
  
    let mut access = TokenType::default();
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut col: FsmTransitions = _Reject;
    let mut prev_state: FsmTransitions = _Reject;
    let mut current_state: FsmTransitions = _Reject;
    let mut current_token = String::new();

    //loop through characters
    for c in expression.chars() {

        col = get_col( c );
       // println!("Collum is {} .", col as i32);
       current_state = STATE_TABLE[current_state as usize][col as usize];

       //for reject
       if current_state == _Reject  {

           
           if prev_state != _Space{

               access.token = current_token.clone();
               access.lexeme = prev_state;
               access.lexeme_name = get_lexeme_name( &access.lexeme );
               tokens.push( access.clone() );
           }
           current_token = "".to_string();
       } else {
           current_token.push( c );
       } 
       
       prev_state = current_state;
    }

    if current_state != _Space && current_token != "" {
        access.token = current_token.clone();
        access.lexeme = current_state;
        access.lexeme_name = get_lexeme_name( &access.lexeme );
        tokens.push( access.clone());
    }

   tokens
    

}

fn get_col(c: char) -> FsmTransitions {

    if c == ' ' {

       _Space

   } else if c.is_digit(10) {

       _Integer

   } else if c == '.' {

       _Real

   } else if c.is_alphabetic() {

       _String

   } else if c.is_ascii_punctuation() {

       _Operator
   } else {

       _Unknown
   }


}

fn get_lexeme_name( lexeme: &FsmTransitions ) -> String {

    match lexeme {

        _Space    => "SPACE".to_string(),
        _Integer  => "INTEGER".to_string(),
        _Real     => "REAL".to_string(),
        _String   => "STRING".to_string(),
        _Operator => "OPERATOR".to_string(),
        _Unknown  => "UNKNOWN".to_string(),
        _         => "ERROR".to_string(),
    }
}
