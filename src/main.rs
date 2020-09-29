#[path = "file_handling/file_handling.rs"]
mod file_handling;
//use file_handling::tokens::*;

fn main() {
    let mut count: i32 = 0;
    let expression = file_handling::convert_file_to_string( file_handling::get_file_name( &mut count ) );
    let token: Vec< file_handling::lexer::TokenType> = file_handling::lexer::lexer(&expression);

    file_handling::write_to_file( file_handling::get_file_name(&mut count), token );
    
}
