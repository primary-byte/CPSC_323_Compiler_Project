use std::io;
use std::env;
use std::iter::Peekable;

//This enum will contains possible types of grammar that we wish to extract
#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Sub,
    Div,
    Number(u64),
    Paren
}

//this is a node in the parse tree
//CONTAINS: 1) children (vector of children nodes)
//          2) Grammar Item (type/value of grammar the node contains)
#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

//initialize the parsenode to an empty children vector and a parenthesis (like a $ in lecture)
impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

//this is any possible item we lex out of the input string
#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

//lex stuff - low priority since we already have one
fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

//converts string numbers to 64 bit unsigned itneger type
fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut number = c.to_string().parse::<u64>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

//first recursive parser function - HELPER FUNCTION this one calls the others
//take in string, returns the parsenode and string inside of a result (for error handling)
pub fn parse(input: &String) -> Result<ParseNode, String> {
    //lex input string
    let tokens = lex(input)?;
    //call the parse_expr function with the list of tokens and 0 for the root node.
    //error handling will be called if number of parseNodes does not equal the number of tokens
    parse_expr(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
        Ok(n)
    } else {
        Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    })
}

//true recursive parser funcction
//Takes tokens (Vector of LexItem), and current position (node level)
//Returns Result of ParseNode and position (node level) as well as string for error handling
fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {

    //parse summand first
    let (node_summand, next_pos) = parse_summand(tokens, pos)?;

    //get next token if we parsed the summand
    let c = tokens.get(next_pos);

    //match kind of operator
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the addition expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            //recurse the right side now
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }Some(&LexItem::Op('-'))=>{
            //recurse of the negative expr
            let mut sub = ParseNode::new();
            sub.entry = GrammarItem::Sub;
            sub.children.push(node_summand);
            //recurse right side now
            let (rhs, i) = parse_expr(tokens, next_pos+1)?;
            sub.children.push(rhs);
            Ok((sub, i))
        }
        _ => {
            // we have just the summand production, nothing more.
            Ok((node_summand, next_pos))
        }
    }
}

//parse any summand lexItems (recursively)
//takes vector of tokens (LexItem) amd current position (node level)
//Returns Result of ParseNode and position (node level) as well as string for error handling
fn parse_summand(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    //call parse term 
    let (node_term, next_pos) = parse_term(tokens, pos)?;

    //go to next token
    let c = tokens.get(next_pos);
    match c {
        //match further operations for multiply
        Some(&LexItem::Op('*')) => {
            // recurse on the summand
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_term);
            //recurse for right side
            let (rhs, i) = parse_summand(tokens, next_pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => {
            // we have just the term production, nothing more.
            Ok((node_term, next_pos))
        }
    }
}

//parse any terminal
//takes vector of tokens (LexItem) amd current position (node level)
//Returns Result of ParseNode and position (node level) as well as string for error handling
fn parse_term(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    //get match current LexItem into C or give an error
    let c: &LexItem = tokens.get(pos)
        .ok_or(String::from("Unexpected end of input, expected paren or number"))?;
    match c {
        //for a number terinal we do this
        &LexItem::Num(n) => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Number(n);  
            //recurse
            Ok((node, pos + 1))
        }
        //for a parenthesis terimal we do this
        &LexItem::Paren(c) => {
            match c {
                //any parenthesis
                '(' | '[' | '{' => {
                    //parse any remaining expressions then do the following
                    parse_expr(tokens, pos + 1).and_then(|(node, next_pos)| {
                        //match the parenthesis with the matching() helper function
                        if let Some(&LexItem::Paren(c2)) = tokens.get(next_pos) {
                            if c2 == matching(c) {
                                // okay!
                                let mut paren = ParseNode::new();
                                paren.children.push(node);
                                //recurse 
                                Ok((paren, next_pos + 1))
                            } else {
                                //parenthesis not matching
                                Err(format!("Expected {} but found {} at {}",
                                            matching(c),
                                            c2,
                                            next_pos))
                            }
                        } else {
                            //other character match error
                            Err(format!("Expected closing paren at {} but found {:?}",
                                        next_pos,
                                        tokens.get(next_pos)))
                        }
                    })
                }
                //base error case
                _ => Err(format!("Expected paren at {} but found {:?}", pos, c)),
            }
        }
        _ => {
            //outer base error case
            Err(format!("Unexpected token {:?}, expected paren or number", {
                c
            }))
        }
    }
}

//helper matching function
fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("should have been a parenthesis!"),
    }
}
//mostly a stub function to call the others
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        println!("{:?}", parse(&args[1]));
    }
}