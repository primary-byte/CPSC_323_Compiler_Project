use crate::file_handling::lexer::*;
use std::fs::OpenOptions;
use std::io::Write; //string operations
use std::collections::hash_map; //hashmapping

//derive operations to perform deep copies of the enum later
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Symbols{
    //Terminals
    PLUS = 0,           // +
    MINUS = 1,          // - 
    MULT = 2,           // *
    DIV = 3,            // /
    L_PAREN = 4,        // (
    R_PAREN = 5,        // )
    NUM = 6,            // NUM
    ID = 7,             // ID
    END_OF_STACK = 8,   // $

    //Non-Terminals                                 RULE#
    EXPR = 9,           // TE'                          1
    EXPR_PRIME = 10,    // +TE' | -TE' | EPSILON        2,3,4
    TERM = 11,          // F | T'                       5,6
    TERM_PRIME = 12,    // *FT' | /FT'                  7,8
    FACTOR = 13,        // (E) | ID | <NUM>             9,10,11
    ID_NT = 14          // id                           12

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

    //create hash map
    let mut LL_TABLE = hash_map::new();

    //create EXPR row
    LL_TABLE.insert((Symbols::EXPR, Symbols::L_PAREN), 1);
    LL_TABLE.insert((Symbols::EXPR, Symbols::ID), 1);
    LL_TABLE.insert((Symbols::EXPR, Symbols::NUM), 1);
    LL_TABLE.insert((Symbols::EXPR_PRIME, Symbols::PLUS), 2);
    LL_TABLE.insert((Symbols::EXPR_PRIME, Symbols::MINUS), 3);
    LL_TABLE.insert((Symbols::TERM, Symbols::L_PAREN), 5);
    LL_TABLE.insert((Symbols::EXPR_PRIME, Symbols::PLUS), 2);       

    //create symbol stack
    let mut ss = Vec!<Symbols>;

    let mut token_pointer: usize = 0;

    //push end of stack $
    ss.push(Symbols::END_OF_STACK);
    //push expression
    ss.push(Symbols::EXPR);

    while(ss.size() > 0){
        //compare the lexer at pointer to stack
        if(ss.front() == lexer_to_symbol(token_list[token_pointer]){
            
            println!("Match symbols: {}", lexer_to_symbol(token_list[token_pointer]));
            
            //increment token pointer
            token_pointer = token_pointer + 1;

            //pop off front of vector stack
            ss.pop();
        }
        else
        {

            let mut nlength = ss.len() - 1;
            //holds current rule cell (usize)
            let mut current_table_cell = LL_TABLE[ss[nlength]][lexer_to_symbol(token_list[token_pointer])];

            //output the rule
            println!("Rule: {}", current_table_cell
            );

            //match correct rule
            match current_table_cell{
                //TE'
                1 => {
                    //remvove front
                    ss.pop();
                    ss.push(Symbols::TERM);
                    ss.push(Symbols::EXPR_PRIME);
                }

                //+TE'
                2 => {
                    //remove front
                    ss.pop();
                    //see above
                    ss.push(Symbols::PLUS);
                    ss.push(Symbols::TERM);
                    SS.push(Symbols::EXPR_PRIME);
                    
                    
                }

                //-TE'
                3 => {
                    ss.pop();
                    ss.push(Symbols::MINUS);
                    ss.push(Symbols::TERM);
                    SS.push(Symbols::EXPR_PRIME);
                    
                    
                }

                //EPSILON
                4 => {
                    
                    
                    
                }

                //F
                5 => {
                    ss.pop();
                    ss.push(Symbols::FACTOR);
                    
                }

                //T'
                6 => {
                    ss.pop();
                    ss.push(Symbols::TERM_PRIME);

                }
                
                //*FT'
                7 => {

                    ss.pop();
                    ss.push(Symbols::MULT);
                    ss.push(Symbols::FACTOR);
                    ss.push(Symbols::TERM_PRIME);
                }

                ///FT'
                8 => {

                    ss.pop();
                    ss.push(Symbols::FACTOR);
                    ss.push(Symbols::TERM_PRIME);
                }

                //(E)
                9 => {
                    //remove front
                    ss.remove(0);
                    //add r_paren, expr, l_paren
                    ss.push(Symbols::R_PAREN);
                    ss.push(Symbols::EXPR);
                    ss.push(Symbols::L_PAREN);
                }

                //ID_NT
                10 => {
                    ss.pop();
                    ss.push(Symbols::ID_NT);
                }

                // <NUM>
                11 => {
                    ss.pop();
                    ss.push(Symbols::NUM);
                }

                //id
                12 => {
                    ss.pop();
                    ss.push(Symbols::ID);
                }

                //default
                _=>{
                    //error out
                    Error("Defaulted out and died. RIP");
                }
            }



        }
        
    }

}