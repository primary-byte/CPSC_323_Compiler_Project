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
    pub rule: String,
}

//implement constructor for our parse node within the parse tree
impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: "(".to_string(),
            //default to declarative
            rule:"<Statement> -> <Declarative> ".to_string(),
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

    parse_declarative(&token_list, 0).and_then(|(result_list, iterations)| {
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
fn parse_declarative(
    token_list: &Vec<TokenType>,
    position: usize,
) -> Result<(ParseNode, usize), String> {
    let (node_assign, next_position) = parse_assignment(token_list, position)?;

    if next_position < token_list.len() {
        let current_token = &token_list[next_position];

        
        match current_token.lexeme_name.as_str() {
            "IDENTIFIER" => {
                let mut node_declar = ParseNode::new();
                node_declar.entry = current_token.token.clone();
                //ID rule
                node_declar.rule = "<Assign> -> <ID> = <Expression>".to_string();
                node_declar.children.push(node_assign);

                Ok((node_declar, next_position + 1))
            }
            _ => Ok((node_assign, next_position + 1)),
        }
    } else {
        Ok((node_assign, next_position))
    }
}

fn parse_assignment(
    token_list: &Vec<TokenType>,
    position: usize,
) -> Result<(ParseNode, usize), String> {
    let (node_expression, next_position) = parse_expression(token_list, position)?;

    if next_position < token_list.len() {
        let current_token = &token_list[next_position];

        match current_token.token.as_str() {
            "=" => {
                let mut assign_node = ParseNode::new();
                assign_node.entry = '='.to_string();
                assign_node.rule = "<Statement> -> <Assign>".to_string();
                assign_node.children.push(node_expression);

                let (right_side, new_position) = parse_assignment(token_list, next_position + 1)?;
                assign_node.children.push(right_side);
                Ok((assign_node, new_position))
            }
            _ => Ok((node_expression, next_position)),
        }
    } else {
        Ok((node_expression, next_position))
    }
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
                sum_node.rule = "<Expression> -> <Expression> + <Term> | <Expression> - <Term> | <Term> ".to_string();
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
                minus_node.rule = "<Expression> -> <Expression> + <Term> | <Expression> - <Term> | <Term> ".to_string();
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
                mult_node.rule = "<Term> -> <Term> * <Factor> | <Term> / <Factor> | <Factor>".to_string();
                mult_node.children.push(node_terminal);
                let (right_side, new_position) = parse_summand(token_list, next_position + 1)?;
                mult_node.children.push(right_side);
                Ok((mult_node, new_position))
            }
            "/" => {
                //recuse on summand again
                let mut div_node = ParseNode::new();
                div_node.entry = '/'.to_string();
                div_node.rule = "<Term> -> <Term> * <Factor> | <Term> / <Factor> | <Factor>".to_string();
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

    // DEBUG
    // println!("{}", current_token.lexeme_name.as_str());
    match current_token.lexeme_name.as_str() {
        "KEYWORD" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();
            node.rule = "<Type> -> bool | float | int".to_string();
            Ok((node, position + 1))
        }
        "IDENTIFIER" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();
            node.rule = "<ID> -> id".to_string();
            Ok((node, position + 1))
        }
        "INTEGER" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();
            node.rule = "<Factor> -> ( <Expression> ) | <ID> | <Num>".to_string();
            Ok((node, position + 1))
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

//recursive pretty print function
pub fn print_tree(node: &ParseNode){

    //overload this for beauty reasons
    fn print_tree(node: &ParseNode, prefix: String, last_node: bool){
        
        //check last node for end prefix
        let prefix_current = if last_node {"- "} else { "| - "};

        //print the good stuff
        println!("{}{}{}", prefix, prefix_current, node.entry);
        println!("{}{}rule: {}",prefix, prefix_current, node.rule);

        //prefix logic
        let prefix_child = if last_node {"  " } else {"| "};
        let prefix = prefix + prefix_child;

        //if we aren't empty call more
        if !node.children.is_empty(){

            //check last child node
            let last_child = node.children.len() - 1;

            //DO ITTTTTT
            for (i, child) in node.children.iter().enumerate(){
                print_tree(&child, prefix.to_string(), i == last_child)
            }

        }
    }
    //run the recursion
    print_tree(node, "".to_string(), true);
}


