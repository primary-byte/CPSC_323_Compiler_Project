#[path = "../tokens/tokens.rs"]
use crate::file_handling::lexer::*;
use std::iter::Peekable; //for peeking ahead without popping


//contains grammar items in our language TO REMOVE LATER
//#[derive(Debug, Clone)]
/*pub enum GrammarItem {
    Product,
    Sum,
    Sub,
    Div,
    Number(u64),
    Paren,
}*/

//node in the parse tree
#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: String,
}

//implement constructor for our parse node within the parse tree
impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: "(".to_string(),
        }
    }
}


//main parse function. This function will call the recursive functions 
//  which will call other grammar recursion functions as needed.
//  Input: Vector of tokens (token_list)
//  Output: Result tuple of ParseNode (a parse tree) and an error message if necessary 
//  via the Err builtin function
pub fn parse(token_list: &Vec<TokenType>) -> Result<ParseNode,  String> {
    //start at root node and call recursive function to parse expressions
    
    parse_expression(&token_list, 0).and_then(|(resultList,iterations)|{
    //check to see we parsed the whole list successfully
        if iterations == token_list.len(){
            Ok(resultList)
        }
    //error if we did not parse successfully
        else {
            Err(format!(
                "Expected end of input", 
            ))
        }
    })
}


//expression parse function. This is the first recursive function to be called and will call the others as needed
//  Input: Vector of tokens (token_list), Position in list
//  Output: Result tuple of ParseNode (a parse tree), position in list, and an error message if necessary 
//  via the Err builtin function
fn parse_expression(token_list: &Vec<TokenType>, position: usize) -> Result<(ParseNode, usize), String>{
    //parse the first id or summand
    //if an id then parse_summand will handle this also
    let (node_summand, next_position) = parse_summand(token_list, position)?;

    //get current working token, then match it
    let currentToken = &token_list[position + 1];
    match currentToken.token.as_str() {
        "+" => {
            //recurse on the expression
            //create new + node
            let mut sumNode = ParseNode::new();
            sumNode.entry = '+'.to_string();
            //push onto vector/stack
            sumNode.children.push(node_summand);
            //recurse time!
            //Note: ? will abbreviate error handling to call the Err function if Error returned, and the Ok function if the Result is OK
            let (rightSide, newPosition) = parse_expression(token_list, next_position + 1)?;
            sumNode.children.push(rightSide);
            Ok((sumNode, newPosition))
        }
        "-" => {
            //recurse on the expression
            //create new - node
            let mut minusNode = ParseNode::new();
            minusNode.entry = '-'.to_string();
            //push onto vector/stack
            minusNode.children.push(node_summand);
            //recurse time!
            let (rightSide, newPosition) = parse_expression(token_list, next_position + 1)?;
            minusNode.children.push(rightSide);
            Ok((minusNode, newPosition))
        }
        _=> {
            //base case we are done here
            Ok((node_summand, next_position))
        }
    }
}

//summand/id parse function. This is the second recursive function to be called and will call the others as needed
//  Input: tupleVector of tokens (token_list), Position in list
//  Output: Result tuple of ParseNode (a parse tree), position in list, and an error message if necessary 
//  via the Err builtin function
fn parse_summand(token_list: &Vec<TokenType>, position: usize) -> Result<(ParseNode, usize), String> {
 
    //recursive parse terminals
    let (node_terminal, next_position) = parse_terminal(token_list, position)?;
    //work on next token
    let currentToken = &token_list[position+1];
    match currentToken.token.as_str(){
        "*" =>{
            //recuse on summand again
            let mut multNode = ParseNode::new();
            multNode.entry = '*'.to_string();
            multNode.children.push(node_terminal);
            let (rightSide, newPosition) = parse_summand(token_list, next_position + 1)?;
            multNode.children.push(rightSide);
            Ok((multNode, newPosition))
        }
        "/" =>{
            //recuse on summand again
            let mut divNode = ParseNode::new();
            divNode.entry = '/'.to_string();
            divNode.children.push(node_terminal);
            let (rightSide, newPosition) = parse_summand(token_list, next_position + 1)?;
            divNode.children.push(rightSide);
            Ok((divNode, newPosition))
        }
        _=> {
            //base case we are done here
            Ok((node_terminal, next_position))
        }
    }
}

// terminal parse function. This is the third recursive function to be called and will call the others as needed
//  Input: Vector of tokens (token_list), Position in list
//  Output: Result tuple of ParseNode (a parse tree), position in list, and an error message if necessary 
//  via the Err builtin function
fn parse_terminal(token_list: &Vec<TokenType>, position: usize) -> Result<(ParseNode, usize), String>{
    //get current token or error message
    let currentToken: &TokenType = token_list.get(position).ok_or(String::from("Unexpected end of input, expected parenthesis or number",))?;
    
    if is_string_numeric(currentToken.token.clone()) {
    let mut node = ParseNode::new();
    node.entry = currentToken.token.clone();
    Ok((node, position))
    } else if currentToken.token.as_str() == "(" {
        parse_expression(token_list, position + 1).and_then(|(node, next_pos)| {
             if token_list[next_pos].token.as_str() == ")" {
                let mut paren = ParseNode::new();
                paren.children.push(node);
                Ok((paren, next_pos +1))
             }else {
                 Err(format!(
                     "Expected {} but found {} at {}",
                     ")",
                     token_list[next_pos].token.as_str(),
                     next_pos
                 ))
             }
        })
    }else {
        Err(format!(
            "Expected closing paren at {} but found {:?}",
            position,
            token_list[position+1].token.as_str()
        ))
    }

    
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

