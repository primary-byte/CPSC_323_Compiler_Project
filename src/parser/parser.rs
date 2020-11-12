#[path = "../tokens/tokens.rs"]
pub mod tokens;
pub use tokens::*;


pub fn parse(token_list: &Vec<TokenType>) -> String {
    let recursive_stack = &mut Vec<TokenType>
}