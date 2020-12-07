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
    END_OF_STACK, // $

    //Non-Terminals                                 RULE#
    EXPR,       // TE'                          1
    EXPR_PRIME, // +TE' | -TE' | EPSILON        2,3,4
    TERM,       // F | T'                       5,6
    TERM_PRIME, // *FT' | /FT'                  7,8
    FACTOR,     // (E) | ID | <NUM>             9,10,11
    ID_NT,      // id                           12
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
    LL_TABLE.insert((EXPR, L_PAREN), 1);
    LL_TABLE.insert((EXPR, ID), 1);
    LL_TABLE.insert((EXPR, NUM), 1);
    LL_TABLE.insert((EXPR_PRIME, PLUS), 2);
    LL_TABLE.insert((EXPR_PRIME, MINUS), 3);
    LL_TABLE.insert((EXPR_PRIME, R_PAREN), 4);
    LL_TABLE.insert((EXPR_PRIME, END_OF_STACK), 4);
    LL_TABLE.insert((TERM, L_PAREN), 5);
    LL_TABLE.insert((TERM, NUM), 5);
    LL_TABLE.insert((TERM, ID), 5);
    LL_TABLE.insert((TERM_PRIME, PLUS), 2);
    LL_TABLE.insert((FACTOR, L_PAREN), 9);
    LL_TABLE.insert((FACTOR, ID), 10);
    LL_TABLE.insert((ID_NT, ID), 12);

    //create symbol stack
    let mut ss: Vec<Symbols> = Vec::new();

    let mut token_pointer: usize = 0;

    //push end of stack $
    ss.push(END_OF_STACK);
    //push expression
    ss.push(EXPR);

    while ss.len() > 1 {
        //debug vector size
        ss.shrink_to_fit();
        println!("Vector in now len: {:?}", ss.len());
        //compare the lexer at pointer to stack
        if ss[ss.len() - 1] == lexer_to_symbol(&token_list[token_pointer]) {
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
                    ss.push(Symbols::MULT);
                    ss.push(Symbols::FACTOR);
                    ss.push(Symbols::TERM_PRIME);
                }

                //FT'
                Some(8) => {
                    ss.pop();
                    ss.push(Symbols::FACTOR);
                    ss.push(Symbols::TERM_PRIME);
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

                //default
                _ => {
                    println!("ERROR!");
                }
            }
        }
    }
}
