use crate::file_handling::lexer::*;
use std::fs::OpenOptions;
use std::io::Write; //string operations

//node in the parse tree
#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: String,
    pub rule: Vec<String>,
    pub token: TokenType,
}

//implement constructor for our parse node within the parse tree
impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: "$".to_string(),
            //default to declarative
            rule: Vec::new(),
            token: tokens::TokenType {
                token: "".to_string(),
                lexeme: fsm::_Reject,
                lexeme_name: "".to_string(),
            },
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

    //create stack of rule strings
    let mut rule_vector: Vec<String> = Vec::new();
    let mut root_node = ParseNode::new();
    root_node.rule.push("Sart".to_string());

    parse_declarative(&token_list, 0, &mut rule_vector).and_then(|(mut result_list, iterations)| {
        //check to see we parsed the whole list successfully
        if iterations == token_list.len() {
            Ok(result_list)
        } else if iterations < token_list.len() {
            root_node.children.push(result_list);
            let mut new_position = iterations;

            while new_position < token_list.len() {
                let (additional_result, new_new_position) =
                    parse_declarative(&token_list, new_position, &mut rule_vector)?;

                new_position = new_new_position;

                root_node.children.push(additional_result);
                //rule_vector.clear();
            }
            Ok(root_node)
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
    //hold rules so far
    rules_so_far: &mut Vec<String>,
) -> Result<(ParseNode, usize), String> {
    let (node_assign, next_position) = parse_assignment(token_list, position, rules_so_far)?;

    if next_position < token_list.len() {
        let current_token = &token_list[next_position];

        match current_token.lexeme_name.as_str() {
            "IDENTIFIER" => {
                let mut node_declar = ParseNode::new();
                node_declar.entry = current_token.token.clone();
                //ID rule
                //add rule
                rules_so_far.clear();
                rules_so_far.push("<Declarative> -> <Type> <ID>".to_string());
                node_declar.rule = rules_so_far.clone();
                node_declar.token = current_token.clone();
                node_declar.children.push(node_assign);

                Ok((node_declar, next_position + 1))
            }
            _ => Ok((node_assign, next_position)),
        }
    } else {
        Ok((node_assign, next_position))
    }
}

fn parse_assignment(
    token_list: &Vec<TokenType>,
    position: usize,
    //hold rules so far
    rules_so_far: &mut Vec<String>,
) -> Result<(ParseNode, usize), String> {
    let (node_expression, next_position) = parse_expression(token_list, position, rules_so_far)?;

    if next_position < token_list.len() {
        let current_token = &token_list[next_position];

        match current_token.token.as_str() {
            "=" => {
                let mut assign_node = ParseNode::new();
                assign_node.entry = '='.to_string();
                rules_so_far.push("<Assign> -> <ID> = <Expression>".to_string());
                assign_node.rule = rules_so_far.clone();
                assign_node.children.push(node_expression);

                let (right_side, new_position) =
                    parse_assignment(token_list, next_position + 1, rules_so_far)?;
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
    rules_so_far: &mut Vec<String>,
) -> Result<(ParseNode, usize), String> {
    //parse the first id or summand
    //if an id then parse_summand will handle this also
    let (node_summand, next_position) = parse_summand(token_list, position, rules_so_far)?;

    //get current working token, then match it
    if next_position < token_list.len() {
        let current_token = &token_list[next_position];
        match current_token.token.as_str() {
            "+" => {
                //recurse on the expression
                //create new + node
                let mut sum_node = ParseNode::new();
                sum_node.entry = '+'.to_string();
                rules_so_far.clear();
                rules_so_far.push("<ExpressionPrime> -> +<Term> <ExpressionPrime>".to_string());
                sum_node.rule = rules_so_far.clone();
                sum_node.token = current_token.clone();
                //push onto vector/stack
                sum_node.children.push(node_summand);
                //recurse time!
                //Note: ? will abbreviate error handling to call the Err function if Error returned, and the Ok function if the Result is OK
                let (right_side, new_position) =
                    parse_expression(token_list, next_position + 1, rules_so_far)?;
                sum_node.children.push(right_side);
                Ok((sum_node, new_position))
            }
            "-" => {
                //recurse on the expression
                //create new - node
                let mut minus_node = ParseNode::new();
                minus_node.entry = '-'.to_string();
                rules_so_far.clear();
                rules_so_far.push("<ExpressionPrime> -> -<Term> <ExpressionPrime>".to_string());
                minus_node.rule = rules_so_far.clone();
                minus_node.token = current_token.clone();
                //push onto vector/stack
                minus_node.children.push(node_summand);
                //recurse time!
                let (right_side, new_position) =
                    parse_expression(token_list, next_position + 1, rules_so_far)?;
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
    rules_so_far: &mut Vec<String>,
) -> Result<(ParseNode, usize), String> {
    //recursive parse terminals
    let (node_terminal, next_position) = parse_terminal(token_list, position, rules_so_far)?;
    //work on next token
    if next_position < token_list.len() {
        let current_token = &token_list[next_position];
        match current_token.token.as_str() {
            "*" => {
                //recuse on summand again
                let mut mult_node = ParseNode::new();
                mult_node.entry = '*'.to_string();
                rules_so_far.clear();
                rules_so_far.push("<TermPrime> ->  *<Factor> <TermPrime>".to_string());
                mult_node.rule = rules_so_far.clone();
                mult_node.token = current_token.clone();
                mult_node.children.push(node_terminal);
                let (right_side, new_position) =
                    parse_summand(token_list, next_position + 1, rules_so_far)?;
                mult_node.children.push(right_side);
                Ok((mult_node, new_position))
            }
            "/" => {
                //recuse on summand again
                let mut div_node = ParseNode::new();
                div_node.entry = '/'.to_string();
                rules_so_far.clear();
                rules_so_far.push("<TermPrime> ->  /<Factor> <TermPrime>".to_string());
                div_node.rule = rules_so_far.clone();
                div_node.token = current_token.clone();
                div_node.children.push(node_terminal);
                let (right_side, new_position) =
                    parse_summand(token_list, next_position + 1, rules_so_far)?;
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
    rules_so_far: &mut Vec<String>,
) -> Result<(ParseNode, usize), String> {
    //get current token or error message
    let current_token: &TokenType = token_list.get(position).ok_or(String::from(
        "Unexpected end of input, expected parenthesis or number",
    ))?;

    match current_token.lexeme_name.as_str() {
        "KEYWORD" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();

            match current_token.token.as_str() {
                "bool" => rules_so_far.push("<Type> -> bool".to_string()),
                "float" => rules_so_far.push("<Type> -> float".to_string()),
                "int" => rules_so_far.push("<Type> -> int".to_string()),
                _ => println!("Error 264"),
            }
            node.rule = rules_so_far.clone();
            node.token = current_token.clone();
            Ok((node, position + 1))
        }
        "IDENTIFIER" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();

            rules_so_far.push("<ID> -> id".to_string());
            node.rule = rules_so_far.clone();
            node.token = current_token.clone();
            Ok((node, position + 1))
        }
        "INTEGER" => {
            let mut node = ParseNode::new();
            node.entry = current_token.token.clone();
            rules_so_far.clear();
            rules_so_far.push("<factor> -> <num>".to_string());
            node.rule = rules_so_far.clone();
            node.token = current_token.clone();
            Ok((node, position + 1))
        }
        "SEPARATOR" => {
            parse_expression(token_list, position + 1, rules_so_far).and_then(|(node, next_pos)| {
                if token_list[next_pos].token.as_str() == ")" {
                    let mut paren = ParseNode::new();
                    paren.entry = current_token.token.clone();
                    rules_so_far.push("<Factor> -> ( <Expression> )".to_string());
                    paren.rule = rules_so_far.clone();
                    paren.token = token_list[next_pos].clone();
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
            })
        }
        _ => Err(format!(
            "Expected closing paren at {} but found {:?}",
            position,
            token_list[position + 1].token.as_str()
        )),
    }
}

//recursive pretty print function
pub fn print_tree(file_name: &String, node: &ParseNode) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_name.trim())
        .unwrap();

    //overload this for beauty reasons
    fn print_tree(
        mut file: &std::fs::File,
        file_name: &String,
        node: &ParseNode,
        prefix: String,
        last_node: bool,
    ) {
        //check last node for end prefix
        let prefix_current = if last_node { "- " } else { "| - " };

        //get string from the node rule string vector
        let mut node_rule_string = "".to_string();

        //iterate over rules and print out the right rules to a new string
        for rule in node.rule.iter().rev() {
            //node_rule_string.push('\n');
            node_rule_string = node_rule_string + rule;
        }

        //print the good stuff
        let mut line = format!("{}{}{}", prefix, prefix_current, node.entry);
        if let Err(e) = writeln!(file, "{:?}", line) {
            eprintln!("Could not write to file: {}", e);
        }

        line = format!("{}{}Token: {:?}", prefix, prefix_current, node.token);
        if let Err(e) = writeln!(file, "{:?}", line) {
            eprintln!("Could not write to file: {}", e);
        }

        line = format!("{}{}Rule: {}", prefix, prefix_current, node_rule_string);
        if let Err(e) = writeln!(file, "{:?}", line) {
            eprintln!("Could not write to file: {}", e);
        }

        //prefix logic
        let prefix_child = if last_node { "  " } else { "| " };
        let prefix = prefix + prefix_child;

        //if we aren't empty call more
        if !node.children.is_empty() {
            //check last child node
            let last_child = node.children.len() - 1;

            //DO ITTTTTT
            for (i, child) in node.children.iter().enumerate() {
                print_tree(file, file_name, &child, prefix.to_string(), i == last_child)
            }
        }
    }
    //run the recursion
    print_tree(&file, file_name, node, "".to_string(), true);
}
