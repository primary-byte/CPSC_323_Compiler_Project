use crate::file_handling::lexer::*;
use std::fs::OpenOptions;
use std::io::Write; //string operations

//derive operations to perform deep copies of the enum later
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Symbols{
    //Terminals
    PLUS,           // +
    MINUS,          // - 
    MULT,           // *
    DIV,            // /
    L_PAREN,        // (
    R_PAREN,        // )
    NUM,            // NUM
    ID,             // ID
    END_OF_STACK,   // $

    //Non-Terminals                                 RULE#
    EXPR,           // TE'                          1
    EXPR_PRIME,     // +TE' | -TE' | EPSILON        2
    TERM,           // F | T'                       3
    TERM_PRIME,     // *FT' | /FT'                  4
    FACTOR          // (E) | ID | <NUM>             5

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
EXPR	    −	−	−	−	+	−	+	+	−
EXPR_PRME	+	+	−	−	−	+	−	−	+
TERM	    −	−	−	−	+	−	+	+	−
TERMPRIME   +	+	+	+	−	+	−	−	+
FACTOR      −	−	−	−	+	−	+	+	−
End_of_stk  −	−	−	−	−	−	−	+	−






*/

//numbers correspond to rules, see above table
pub const LL_TABLE: &[&[usize:32]] = &[
    //EXPR
    &[
        0,0,0,0,0,0,0,0,0
    ],
    //EXPRE_PRIME
    &[
        0,0,0,0,0,0,0,0,0
    ],
    //TERM
    &[
        0,0,0,0,0,0,0,0,0
    ],
    //TERMPRIME
    &[
        0,0,0,0,0,0,0,0,0
    ],
    //FACTOR
    &[
        0,0,0,0,0,0,0,0,0
    ],
    //END_OF_STACK
    &[
        0,0,0,0,0,0,0,0,0
    ],

];

pub fn lexer_to_symbol(current_token: TokenType) => Symbols{


    //match token to symbol enum
    match current_token.leme_name.as_str(){
        "IDENTIFIER" => {
            //return ID enum symbol
            ID
        }

        "SEPARATOR" => {
            //match against parenthesis
            match current_token.token.as_str(){
                "(" =>{
                    L_PAREN
                }
                ")" =>{
                    R_PAREN  
                }

                _ => {Err(format!(
                    "Expected a prenthesis but did not get one."
                ))
            }
        }

        "OPERATOR" => {
            match current_token.token.as_str(){
                "+" =>{
                    PLUS
                }
                "-" =>{
                    MINUS
                }
                "*" =>{
                    MULT
                }
                "/" =>{
                    DIV
                }

                _ => {Err(format!(
                    "Expected an operator but did not get one."
                    ))
                }
            }
        }

        "INTEGER" => {
            NUM
        }

        _=>{
            Err(format!(
                "Unexpected value into lexer. Token value: {}", current_token.value.as_str()
                ))
        }

    }

}

pub fn parse(token_list: Vec!<TokenType>){

    //create symbol stack
    let mut ss = Vec!<Symbols>;

}