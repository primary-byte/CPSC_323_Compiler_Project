#[path = "file_handling/file_handling.rs"]
mod file_handling;
mod parser;
//use file_handling::tokens::*; not needed right now

fn main() {
    let mut count: i32 = 0; //initial arg count from cli
    //get file contents as a string
    let expression =
        file_handling::convert_file_to_string(file_handling::get_file_name(&mut count));

    //get vector of tokens from the lexer module
    let token: Vec<file_handling::lexer::TokenType> = file_handling::lexer::lexer(&expression);

    //write vector of tokens to specified file
    //file_handling::write_to_file(file_handling::get_file_name(&mut count), token);

    let parse_tree = parser::parse(&token);
    match parse_tree{
        Ok(pt) => print!("HI"),  //parser::print_tree( &file_handling::get_file_name(&mut count), &pt),
        Err(e) => println!("Got an error: {:?}", e)
    }

   
}
