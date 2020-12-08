use crate::file_handling::lexer::*;
use std::collections::HashMap; //hashmapping
use std::fs::OpenOptions;
use std::io::Write; //string operations
use Symbols::*;
//derive operations to perform deep copies of the enum later
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Symbols {
    //Terminals
    PLUS,         // +
    MINUS,        // -
    MULT,         // *
    DIV,          // /
    L_PAREN,      // (
    R_PAREN,      // )
    NUM,          // NUM
    ID,           // ID
    EQUAL,
    INT,
    BOOL,
    FLOAT,
    SEMICOLON,
    END_OF_STACK, // $

    //Non-Terminals                                 RULE#
    EXPR,       // TE'                          1
    EXPR_PRIME, // +TE' | -TE' | EPSILON        2,3,4
    TERM,       // F | T'                       5,6
    TERM_PRIME, // *FT' | /FT' | E              7,8, 13
    FACTOR,     // (E) | ID | <NUM>             9,10,11
    ID_NT,      // id                           12
    STATEMENT,  // S
    ASSIGN,
    DECLAR,
    TYPE,
}

/*
First and Follow and Predict Sets
+ signifies inclusion

First sets
        +   -	*	/	(	)	NUM	id	ε
E	    −	−	−	−	+	−	+	+	−
EPRIME	+	+	−	−	−	−	−	−	+
T	    −	−	−	−	+	−	+	+	−
TPRIME	−	−	+	+	−	−	−	−	+
F	    −	−	−	−	+	−	+	+	−
ID	    −	−	−	−	−	−	−	+	−

Follow sets
        +	-	*	/	(	)	NUM	id	$
E	    −	−	−	−	−	+	−	−	+
EPRIME	−	−	−	−	−	+	−	−	+
T	    +	+	−	−	−	+	−	−	+
TPRIME	+	+	−	−	−	+	−	−	+
F	    +	+	+	+	−	+	−	−	+
ID	    +	+	+	+	−	+	−	−	+


Predict sets
                +	-	*	/	(	)	NUM	id	$
EXPR	        −	−	−	−	+	−	+	+	−
EXPR_PRME	    +	+	−	−	−	+	−	−	+
TERM	        −	−	−	−	+	−	+	+	−
TERMPRIME       +	+	+	+	−	+	−	−	+
FACTOR          −	−	−	−	+	−	+	+	−
End_of_stack    −	−	−	−	−	−	−	+	−






*/

//numbers correspond to rules, see above table
// pub const LL_TABLE: &[&[usize32]] = &[
//     //  +,-,*,/,(,),n,id,$
//     //EXPR
//     &[
//         0,0,0,0,9,0,0,0,0
//     ],
//     //EXPRE_PRIME
//     &[
//         0,0,0,0,0,0,0,0,0
//     ],
//     //TERM
//     &[
//         0,0,0,0,0,0,0,0,0
//     ],
//     //TERMPRIME
//     &[
//         0,0,0,0,0,0,0,0,0
//     ],
//     //FACTOR
//     &[
//         0,0,0,0,0,0,0,0,0
//     ],
//     //END_OF_STACK
//     &[
//         0,0,0,0,0,0,0,0,0
//     ],

// ];

pub fn lexer_to_symbol(current_token: &TokenType) -> Symbols {
    //match token to symbol enum
    match current_token.lexeme_name.as_str() {
        "IDENTIFIER" => match current_token.token.as_str() {
            "$" => END_OF_STACK,
            _ => ID,
        },

        "SEPARATOR" => {
            //match against parenthesis
            match current_token.token.as_str() {
                "(" => L_PAREN,
                ")" => R_PAREN,
                ";" => SEMICOLON,

                _ =>
                //{Err(format!(
                // "Expected a prenthesis but did not get one."
                //))
                {
                    ID
                }
            }
        }

        "OPERATOR" => {
            match current_token.token.as_str() {
                "+" => PLUS,
                "-" => MINUS,
                "*" => MULT,
                "/" => DIV,
                "=" => EQUAL,

                _ =>
                //{Err(format!(
                //  "Expected an operator but did not get one."
                // ))
                {
                    ID
                }
            }
        }

        "INTEGER" => NUM,

        "KEYWORD" => {
            match current_token.token.as_str() {
                "int" => INT,
                "bool" => BOOL,
                "float" => FLOAT,
                _ => INT
            }
        }

        _ => {
            /* Err(format!(
            "Unexpected value into lexer. Token value: {}", current_token.value.as_str()
            )) */
            ID
        }
    }
}

pub fn parse(token_list: Vec<TokenType>) {
    //create hash map
    let mut LL_TABLE = HashMap::new();

    //create EXPR row
    LL_TABLE.insert((STATEMENT, L_PAREN), 1);
    LL_TABLE.insert((STATEMENT, ID), 13);
    LL_TABLE.insert((STATEMENT, INT), 15);
    LL_TABLE.insert((STATEMENT, BOOL), 15);
    LL_TABLE.insert((STATEMENT, FLOAT), 15);
    LL_TABLE.insert((STATEMENT, END_OF_STACK), 4);
    LL_TABLE.insert((ASSIGN, ID), 14);
    LL_TABLE.insert((DECLAR, INT), 16);
    LL_TABLE.insert((DECLAR, BOOL), 16);
    LL_TABLE.insert((DECLAR, FLOAT), 16);
    LL_TABLE.insert((TYPE, INT), 17);
    LL_TABLE.insert((TYPE, BOOL), 18);
    LL_TABLE.insert((TYPE, FLOAT), 19);
    LL_TABLE.insert((EXPR, L_PAREN), 1);
    LL_TABLE.insert((EXPR, ID), 1);
    LL_TABLE.insert((EXPR, NUM), 1);
    LL_TABLE.insert((EXPR_PRIME, PLUS), 2);
    LL_TABLE.insert((EXPR_PRIME, MINUS), 3);
    LL_TABLE.insert((EXPR_PRIME, R_PAREN), 4);
    LL_TABLE.insert((EXPR_PRIME, END_OF_STACK), 4);
    LL_TABLE.insert((EXPR_PRIME, MULT), 6);
    LL_TABLE.insert((EXPR_PRIME, DIV), 6);
    LL_TABLE.insert((EXPR_PRIME, SEMICOLON), 20);
    LL_TABLE.insert((TERM, L_PAREN), 5);
    LL_TABLE.insert((TERM, NUM), 5);
    LL_TABLE.insert((TERM, ID), 5);
    LL_TABLE.insert((TERM, MULT), 6);
    LL_TABLE.insert((TERM, DIV), 6);
    LL_TABLE.insert((TERM_PRIME, PLUS), 2);
    LL_TABLE.insert((TERM_PRIME, MINUS), 3);
    LL_TABLE.insert((TERM_PRIME, MULT), 7);
    LL_TABLE.insert((TERM_PRIME, DIV), 8);
    LL_TABLE.insert((TERM_PRIME, R_PAREN), 4);
    LL_TABLE.insert((TERM_PRIME, END_OF_STACK), 4);
    LL_TABLE.insert((FACTOR, L_PAREN), 9);
    LL_TABLE.insert((FACTOR, ID), 10);
    LL_TABLE.insert((ID_NT, ID), 12);
    LL_TABLE.insert((END_OF_STACK, SEMICOLON), 20);
    //create symbol stack
    let mut ss: Vec<Symbols> = Vec::new();

    let mut token_pointer: usize = 0;

    //push end of stack $
    ss.push(END_OF_STACK);
    //push expression
    ss.push(STATEMENT);

    while ss.len() > 0 {
        //let mut line = String::new();
        //let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("Stack: {:?}", ss);
        //debug vector size
        ss.shrink_to_fit();
        //println!("Vector in now len: {:?}", ss.len());
        if ss[ss.len() - 1] == END_OF_STACK && token_list.len() == token_pointer + 1 {
            println!("Code parsed Successfully!! :)");
            ss.pop();
        }

        //compare the lexer at pointer to stack
        else if ss[ss.len() - 1] == lexer_to_symbol(&token_list[token_pointer]) {
            println!("Match symbols: {:?}", token_list[token_pointer].token);
            //increment token pointer
            token_pointer = token_pointer + 1;

            //pop off front of vector stack
            ss.pop();
        } else {
            let mut nlength = ss.len() - 1;
            //holds current rule cell (usize)
            let mut current_table_cell =
                LL_TABLE.get(&(ss[nlength], lexer_to_symbol(&token_list[token_pointer])));

            //output the rule
            println!("Rule: {:?}", current_table_cell);

            //match correct rule
            match current_table_cell {
                //TE'
                Some(1) => {
                    //remvove front
                    ss.pop();
                    ss.push(Symbols::EXPR_PRIME);
                    ss.push(Symbols::TERM);
                }

                //+TE'
                Some(2) => {
                    //remove front
                    ss.pop();
                    //see above
                    ss.push(Symbols::EXPR_PRIME);
                    ss.push(Symbols::TERM);
                    ss.push(Symbols::PLUS);
                }

                //-TE'
                Some(3) => {
                    ss.pop();

                    ss.push(Symbols::EXPR_PRIME);
                    ss.push(Symbols::TERM);
                    ss.push(Symbols::MINUS);
                }

                //EPSILON
                Some(4) => {
                    ss.pop();
                }

                //F
                Some(5) => {
                    ss.pop();
                    ss.push(Symbols::FACTOR);
                }

                //T'
                Some(6) => {
                    ss.pop();
                    ss.push(Symbols::TERM_PRIME);
                }
                //*FT'
                Some(7) => {
                    ss.pop();
                    ss.push(Symbols::TERM_PRIME);
                    ss.push(Symbols::FACTOR);
                    ss.push(Symbols::MULT);
                }

                //FT'
                Some(8) => {
                    ss.pop();
                    ss.push(Symbols::TERM_PRIME);
                    ss.push(Symbols::FACTOR);
                    ss.push(Symbols::DIV);
                }

                //(E)
                Some(9) => {
                    //remove front
                    ss.pop();
                    //add r_paren, expr, l_paren
                    ss.push(Symbols::R_PAREN);
                    ss.push(Symbols::EXPR);
                    ss.push(Symbols::L_PAREN);
                }

                //ID_NT
                Some(10) => {
                    ss.pop();
                    ss.push(Symbols::ID_NT);
                }

                // <NUM>
                Some(11) => {
                    ss.pop();
                    ss.push(Symbols::NUM);
                }

                //id
                Some(12) => {
                    ss.pop();
                    ss.push(Symbols::ID);
                }

                Some(13) => {
                    //check to see if next token is an "="
                    match token_list[token_pointer + 1].token.as_str() {
                        "=" => {
                            ss.pop();
                            ss.push(ASSIGN);
                        }

                        _ => {
                            ss.pop();
                            ss.push(EXPR);
                        }
                    }
                }

                Some(14) => {
                    ss.pop();
                    ss.push(EXPR);
                    ss.push(EQUAL);
                    ss.push(ID_NT);
                }

                Some(15) => {
                    ss.pop();
                    ss.push(DECLAR);
                }

                Some(16) => {
                    ss.pop();
                    ss.push(ID_NT);
                    ss.push(TYPE);
                }

                Some(17) => {
                    ss.pop();
                    ss.push(INT);
                }

                Some(18) => {
                    ss.pop();
                    ss.push(BOOL);
                }

                Some(19) => {
                    ss.pop();
                    ss.push(FLOAT);
                }

                Some(20) => {
                    ss.push(STATEMENT);
                    println!("Matched symbols: \";\"");
                    token_pointer = token_pointer + 1;
                }

                //default
                _ => {
                    println!("ERROR!");
                }
            }
        }
    }
}
