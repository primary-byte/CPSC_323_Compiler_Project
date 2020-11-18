//#[path = "../tokens/tokens.rs"]
use crate::file_handling::lexer::*;
//use std::iter::Peekable; //for peeking ahead without popping not needed yet

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
pub fn parse(token_list: &Vec<TokenType>) -> Result<ParseNode, String> {
    //start at root node and call recursive function to parse expressions

    parse_expression(&token_list, 0).and_then(|(result_list, iterations)| {
        //check to see we parsed the whole list successfully
        if iterations == token_list.len() {
            Ok(result_list)
        }
        //error if we did not parse successfully
        else {
            Err(format!("Expected end of input",))
        }
    })
}

//expression parse function. This is the first recursive function to be called and will call the others as needed
//  Input: Vector of tokens (token_list), Position in list
//  Output: Result tuple of ParseNode (a parse tree), position in list, and an error message if necessary
//  via the Err builtin function
fn parse_expression(
    token_list: &Vec<TokenType>,
    position: usize,
) -> Result<(ParseNode, usize), String> {
    //parse the first id or summand
    //if an id then parse_summand will handle this also
    let (node_summand, next_position) = parse_summand(token_list, position)?;

    //get current working token, then match it
    if next_position < token_list.len() {
        let current_token = &token_list[next_position];
        match current_token.token.as_str() {
        "+" => {
            //recurse on the expression
            //create new + node
            let mut sum_node = ParseNode::new();
            sum_node.entry = '+'.to_string();
            //push onto vector/stack
            sum_node.children.push(node_summand);
            //recurse time!
            //Note: ? will abbreviate error handling to call the Err function if Error returned, and the Ok function if the Result is OK
            let (right_side, new_position) = parse_expression(token_list, next_position + 1)?;
            sum_node.children.push(right_side);
            Ok((sum_node, new_position))
        }
        "-" => {
            //recurse on the expression
            //create new - node
            let mut minus_node = ParseNode::new();
            minus_node.entry = '-'.to_string();
            //push onto vector/stack
            minus_node.children.push(node_summand);
            //recurse time!
            let (right_side, new_position) = parse_expression(token_list, next_position + 1)?;
            minus_node.children.push(right_side);
            Ok((minus_node, new_position))
        }
        _ => {
            //base case we are done here
            Ok((node_summand, next_position))
        }
        }
    } else {
        Ok((node_summand, next_position))
    }
    
    
}

//summand/id parse function. This is the second recursive function to be called and will call the others as needed
//  Input: tupleVector of tokens (token_list), Position in list
//  Output: Result tuple of ParseNode (a parse tree), position in list, and an error message if necessary
//  via the Err builtin function
fn parse_summand(
    token_list: &Vec<TokenType>,
    position: usize,
) -> Result<(ParseNode, usize), String> {
    //recursive parse terminals
    let (node_terminal, next_position) = parse_terminal(token_list, position)?;
    //work on next token
    if next_position < token_list.len() {
        let current_token = &token_list[next_position];
        match current_token.token.as_str() {
        "*" => {
            //recuse on summand again
            let mut mult_node = ParseNode::new();
            mult_node.entry = '*'.to_string();
            mult_node.children.push(node_terminal);
            let (right_side, new_position) = parse_summand(token_list, next_position + 1)?;
            mult_node.children.push(right_side);
            Ok((mult_node, new_position))
        }
        "/" => {
            //recuse on summand again
            let mut div_node = ParseNode::new();
            div_node.entry = '/'.to_string();
            div_node.children.push(node_terminal);
            let (right_side, new_position) = parse_summand(token_list, next_position + 1)?;
            div_node.children.push(right_side);
            Ok((div_node, new_position))
        }
        _ => {
            //base case we are done here
            Ok((node_terminal, next_position))
        }
    }
    } else {
        Ok((node_terminal, next_position))
    }
    
}

// terminal parse function. This is the third recursive function to be called and will call the others as needed
//  Input: Vector of tokens (token_list), Position in list
//  Output: Result tuple of ParseNode (a parse tree), position in list, and an error message if necessary
//  via the Err builtin function
fn parse_terminal(
    token_list: &Vec<TokenType>,
    position: usize,
) -> Result<(ParseNode, usize), String> {
    //get current token or error message
    let current_token: &TokenType = token_list.get(position).ok_or(String::from(
        "Unexpected end of input, expected parenthesis or number",
    ))?;

    println!("{}", current_token.lexeme_name.as_str());
    match current_token.lexeme_name.as_str() {
        "INTEGER" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();
            Ok((node, position+1))
        }
        "SEPARATOR" => parse_expression(token_list, position + 1).and_then(|(node, next_pos)| {
            if token_list[next_pos].token.as_str() == ")" {
                let mut paren = ParseNode::new();
                paren.children.push(node);
                Ok((paren, next_pos + 1))
            } else {
                Err(format!(
                    "Expected {} but found {} at {}",
                    ")",
                    token_list[next_pos].token.as_str(),
                    next_pos
                ))
            }
        }),
        _ => Err(format!(
            "Expected closing paren at {} but found {:?}",
            position,
            token_list[position + 1].token.as_str()
        )),
    }
}
/* if is_string_numeric(current_token.token.clone()) {
    let mut node = ParseNode::new();
    node.entry = current_token.token.clone();
    Ok((node, position))
    } else if current_token.token.as_str() == "(" {
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


}*/
