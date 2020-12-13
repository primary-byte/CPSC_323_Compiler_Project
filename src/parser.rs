use crate::file_handling;
use crate::file_handling::lexer::*;
use prettytable::{Cell, Row, Table};
use std::collections::HashMap; //hashmapping
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use Symbols::*;
//derive operations to perform deep copies of the enum later
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Symbols {
    //Terminals
    PLUS,    // +
    MINUS,   // -
    MULT,    // *
    DIV,     // /
    L_PAREN, // (
    R_PAREN, // )
    NUM,     // num
    ID,      // id
    EQUAL,
    INT,
    BOOL,
    FLOAT,
    COMMA,
    SEMICOLON,
    IF,
    THEN,
    ELSE,
    ENDIF,
    WHILE,
    DO,
    WHILEEND,
    BEGIN,
    END,
    LTHAN,
    LEQUAL,
    EQUALTO,
    NOTEQUAL,
    GTHANEQUAL,
    GTHAN,
    ERROR,
    END_OF_STACK, // $

    //Non-Terminals                                 RULE#
    EXPR,       // TE'                          1
    EXPR_PRIME, // +TE' | -TE' | EPSILON        2,3,4
    TERM,       // F | T'                       5,6
    TERM_PRIME, // *FT' | /FT' | E              7,8, 13
    FACTOR,     // (E) | ID | <NUM>             9,10,11
    ID_NT,      // ID                         12
    STATEMENT,  // S
    ASSIGN,
    DECLAR,
    TYPE,
    MOREIDS,
    MORESTATEMENTS,
    CONDITIONAL,
    RELOP,
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
                "," => COMMA,

                _ =>
                //{Err(format!(
                // "Expected a prenthesis but did not get one."
                //))
                {
                    ERROR
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
                "<" => LTHAN,
                "<=" => LEQUAL,
                "==" => EQUALTO,
                "<>" => NOTEQUAL,
                ">=" => GTHANEQUAL,
                ">" => GTHAN,

                _ =>
                //{Err(format!(
                //  "Expected an operator but did not get one."
                // ))
                {
                    ERROR
                }
            }
        }

        "INTEGER" => NUM,

        "KEYWORD" => match current_token.token.as_str() {
            "int" => INT,
            "bool" => BOOL,
            "float" => FLOAT,
            "if" => IF,
            "then" => THEN,
            "else" => ELSE,
            "endif" => ENDIF,
            "while" => WHILE,
            "do" => DO,
            "whileend" => WHILEEND,
            "begin" => BEGIN,
            "end" => END,
            _ => ERROR,
        },

        _ => {
            /* Err(format!(
            "Unexpected value into lexer. Token value: {}", current_token.value.as_str()
            )) */
            ERROR
        }
    }
}

pub fn parse(token_list: Vec<TokenType>) {
    //create hash map
    let mut LL_TABLE = HashMap::new();

    //create EXPR row
    LL_TABLE.insert((STATEMENT, L_PAREN), 13);
    LL_TABLE.insert((STATEMENT, NUM), 1);
    LL_TABLE.insert((STATEMENT, ID), 13);
    LL_TABLE.insert((STATEMENT, INT), 15);
    LL_TABLE.insert((STATEMENT, BOOL), 15);
    LL_TABLE.insert((STATEMENT, FLOAT), 15);
    LL_TABLE.insert((STATEMENT, IF), 23);
    LL_TABLE.insert((STATEMENT, ELSE), 4);
    LL_TABLE.insert((STATEMENT, ENDIF), 4);
    LL_TABLE.insert((STATEMENT, WHILE), 24);
    LL_TABLE.insert((STATEMENT, WHILEEND), 4);
    LL_TABLE.insert((STATEMENT, BEGIN), 25);
    LL_TABLE.insert((STATEMENT, END), 4);
    LL_TABLE.insert((STATEMENT, SEMICOLON), 4);
    LL_TABLE.insert((STATEMENT, END_OF_STACK), 4);
    LL_TABLE.insert((MORESTATEMENTS, SEMICOLON), 26);
    LL_TABLE.insert((MORESTATEMENTS, WHILEEND), 4);
    LL_TABLE.insert((MORESTATEMENTS, END), 4);
    LL_TABLE.insert((MOREIDS, COMMA), 36);
    LL_TABLE.insert((MOREIDS, SEMICOLON), 4);
    LL_TABLE.insert((ASSIGN, ID), 14);
    LL_TABLE.insert((DECLAR, INT), 16);
    LL_TABLE.insert((DECLAR, BOOL), 16);
    LL_TABLE.insert((DECLAR, FLOAT), 16);
    LL_TABLE.insert((TYPE, INT), 17);
    LL_TABLE.insert((TYPE, BOOL), 18);
    LL_TABLE.insert((TYPE, FLOAT), 19);
    LL_TABLE.insert((CONDITIONAL, ID), 27);
    LL_TABLE.insert((CONDITIONAL, NUM), 27);
    LL_TABLE.insert((RELOP, LTHAN), 28);
    LL_TABLE.insert((RELOP, LEQUAL), 29);
    LL_TABLE.insert((RELOP, EQUALTO), 30);
    LL_TABLE.insert((RELOP, NOTEQUAL), 31);
    LL_TABLE.insert((RELOP, GTHANEQUAL), 32);
    LL_TABLE.insert((RELOP, GTHAN), 33);
    LL_TABLE.insert((EXPR, L_PAREN), 1);
    LL_TABLE.insert((EXPR, ID), 1);
    LL_TABLE.insert((EXPR, NUM), 1);
    LL_TABLE.insert((EXPR_PRIME, PLUS), 2);
    LL_TABLE.insert((EXPR_PRIME, MINUS), 3);
    LL_TABLE.insert((EXPR_PRIME, R_PAREN), 4);
    LL_TABLE.insert((EXPR_PRIME, END_OF_STACK), 4);
    LL_TABLE.insert((EXPR_PRIME, SEMICOLON), 20);
    LL_TABLE.insert((EXPR_PRIME, MULT), 6);
    LL_TABLE.insert((EXPR_PRIME, DIV), 6);
    LL_TABLE.insert((EXPR_PRIME, THEN), 4);
    LL_TABLE.insert((EXPR_PRIME, ELSE), 4);
    LL_TABLE.insert((EXPR_PRIME, ENDIF), 4);
    LL_TABLE.insert((EXPR_PRIME, LTHAN), 4);
    LL_TABLE.insert((EXPR_PRIME, LEQUAL), 4);
    LL_TABLE.insert((EXPR_PRIME, EQUALTO), 4);
    LL_TABLE.insert((EXPR_PRIME, NOTEQUAL), 4);
    LL_TABLE.insert((EXPR_PRIME, GTHANEQUAL), 4);
    LL_TABLE.insert((EXPR_PRIME, GTHAN), 4);
    LL_TABLE.insert((EXPR_PRIME, DO), 4);
    LL_TABLE.insert((EXPR_PRIME, SEMICOLON), 4);
    LL_TABLE.insert((TERM, L_PAREN), 5);
    LL_TABLE.insert((TERM, NUM), 5);
    LL_TABLE.insert((TERM, ID), 5);
    LL_TABLE.insert((TERM, MULT), 6);
    LL_TABLE.insert((TERM, DIV), 6);
    LL_TABLE.insert((TERM_PRIME, PLUS), 4);
    LL_TABLE.insert((TERM_PRIME, MINUS), 4);
    LL_TABLE.insert((TERM_PRIME, MULT), 7);
    LL_TABLE.insert((TERM_PRIME, DIV), 8);
    LL_TABLE.insert((TERM_PRIME, R_PAREN), 4);
    LL_TABLE.insert((TERM_PRIME, SEMICOLON), 4);
    LL_TABLE.insert((TERM_PRIME, THEN), 4);
    LL_TABLE.insert((TERM_PRIME, ELSE), 4);
    LL_TABLE.insert((TERM_PRIME, ENDIF), 4);
    LL_TABLE.insert((TERM_PRIME, LTHAN), 4);
    LL_TABLE.insert((TERM_PRIME, LEQUAL), 4);
    LL_TABLE.insert((TERM_PRIME, EQUALTO), 4);
    LL_TABLE.insert((TERM_PRIME, NOTEQUAL), 4);
    LL_TABLE.insert((TERM_PRIME, GTHANEQUAL), 4);
    LL_TABLE.insert((TERM_PRIME, GTHAN), 4);
    LL_TABLE.insert((TERM_PRIME, DO), 4);
    LL_TABLE.insert((TERM_PRIME, END_OF_STACK), 4);
    LL_TABLE.insert((FACTOR, L_PAREN), 9);
    LL_TABLE.insert((FACTOR, ID), 10);
    LL_TABLE.insert((FACTOR, NUM), 11);
    LL_TABLE.insert((ID_NT, ID), 12);
    LL_TABLE.insert((END_OF_STACK, SEMICOLON), 21);
    //create symbol stack
    let mut ss: Vec<Symbols> = Vec::new();

    let mut token_pointer: usize = 0;

    //push end of stack $
    ss.push(END_OF_STACK);
    //push expression
    ss.push(STATEMENT);

    let mut ST: Vec<(String, String, usize)> = Vec::new();
    let mut symbol_type: String = " ".to_string();
    let mut symbol_name: String = " ".to_string();
    let mut symbol_flag: bool = false;

    let output_path: String = file_handling::get_file_name(&mut 1);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path.trim())
        .unwrap();

    while ss.len() > 0 {
        //let mut line = String::new();
        //let b1 = std::io::stdin().read_line(&mut line).unwrap();

        //println!("Stack: {:?}", ss);
        //println!("Vector in now len: {:?}", ss.len());

        if ss[ss.len() - 1] == END_OF_STACK && token_list.len() == token_pointer + 1 {
            if let Err(e) = writeln!(file,"Parse successfully :) \n") {
                eprintln!("Could not write to file: {}", e);
            }
            ss.pop();
        }
        //compare the lexer at pointer to stack
        else if ss[ss.len() - 1] == lexer_to_symbol(&token_list[token_pointer]) {
            if let Err(e) = writeln!(file, "Match symbols: {:?}", token_list[token_pointer].token) {
                eprintln!("Could not write to file: {}", e);
            }
            //increment token pointer
            token_pointer = token_pointer + 1;

            //pop off front of vector stack
            ss.pop();
        } else {
            let mut current_symbol = lexer_to_symbol(&token_list[token_pointer]);
            let mut nlength = ss.len() - 1;
            //holds current rule cell (usize)
            let mut current_table_cell = LL_TABLE.get(&(ss[nlength], current_symbol));

            //output the rule
            //println!("Rule: {:?}", current_table_cell);

            //match correct rule
            match current_table_cell {
                //TE'
                Some(1) => {
                    //remvove front
                    if let Err(e) = writeln!(file, "Rule: Expression⟶ Term ExpressionPrime") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(EXPR_PRIME);
                    ss.push(TERM);
                }

                //+TE'
                Some(2) => {
                    if let Err(e) = writeln!(file,"ExpressionPrime⟶ + Term ExpressionPrime") {
                eprintln!("Could not write to file: {}", e);
                    }
                   
                    //remove front
                    ss.pop();
                    //see above
                    ss.push(EXPR_PRIME);
                    ss.push(TERM);
                    ss.push(PLUS);
                }

                //-TE'
                Some(3) => {
                    if let Err(e) = writeln!(file,"ExpressionPrime⟶ - Term ExpressionPrime") {
                eprintln!("Could not write to file: {}", e);
                    }
                
                    ss.pop();

                    ss.push(EXPR_PRIME);
                    ss.push(TERM);
                    ss.push(MINUS);
                }

                //EPSILON
                Some(4) => {
                    if let Err(e) = writeln!(file,"Rule: {:?} ⟶ ϵ", ss[nlength]) {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                }

                //F
                Some(5) => {
                     if let Err(e) = writeln!(file,"Term⟶ Factor TermPrime") {
                eprintln!("Could not write to file: {}", e);
                     }
                    ss.pop();
                    ss.push(TERM_PRIME);
                    ss.push(FACTOR);
                }

                //T'
                Some(6) => {
                    ss.pop();
                    ss.push(TERM_PRIME);
                }
                //*FT'
                Some(7) => {
                    if let Err(e) = writeln!(file,"Rule: TermPrime⟶ * Factor TermPrime") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(TERM_PRIME);
                    ss.push(FACTOR);
                    ss.push(MULT);
                }

                //FT'
                Some(8) => {
                    if let Err(e) = writeln!(file,"Rule: TermPrime⟶ / Factor TermPrime") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(TERM_PRIME);
                    ss.push(FACTOR);
                    ss.push(DIV);
                }

                //(E)
                Some(9) => {
                    if let Err(e) = writeln!(file,"Rule: Factor⟶ ( Expression )") {
                eprintln!("Could not write to file: {}", e);
                    }
                    //remove front
                    ss.pop();
                    //add r_paren, expr, l_paren
                    ss.push(R_PAREN);
                    ss.push(EXPR);
                    ss.push(L_PAREN);
                }

                //ID_NT
                Some(10) => {
                     if let Err(e) = writeln!(file,"Factor⟶ ID") {
                eprintln!("Could not write to file: {}", e);
                     }
                    ss.pop();
                    ss.push(Symbols::ID_NT);
                }

                // <NUM>
                Some(11) => {
                    if let Err(e) = writeln!(file,"Rule: Factor⟶ num") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(Symbols::NUM);
                }

                //id
                Some(12) => {
                    if let Err(e) = writeln!(file,"Rule: ID⟶ id") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(Symbols::ID);
                    symbol_name = token_list[token_pointer].token.to_string();
                    if symbol_flag {
                        ST.push((
                            symbol_type.clone(),
                            symbol_name.clone(),
                            token_list[token_pointer].line,
                        ));
                        symbol_flag = false;
                    }
                }

                Some(13) => {
                    //check to see if next token is an "="
                    match token_list[token_pointer + 1].token.as_str() {
                        "=" => {
                             if let Err(e) = writeln!(file,"Rule: Statement⟶ Assign") {
                eprintln!("Could not write to file: {}", e);
                             }
                            ss.pop();
                            ss.push(ASSIGN);
                        }

                        _ => {
                            if let Err(e) = writeln!(file,"Rule: Statement⟶ Expression") {
                eprintln!("Could not write to file: {}", e);
                            }
                            ss.pop();
                            ss.push(EXPR);
                        }
                    }
                }

                Some(14) => {
                    if let Err(e) = writeln!(file,"Assign⟶ ID = Expression") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(EXPR);
                    ss.push(EQUAL);
                    ss.push(ID_NT);
                }

                Some(15) => {
                    if let Err(e) = writeln!(file,"Rule: Statement ⟶ Declarative") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    symbol_type = return_enum_string(current_symbol);
                    ss.push(DECLAR);
                    symbol_flag = true;
                }

                Some(16) => {
                    if let Err(e) = writeln!(file,"Rule: Declarative⟶ Type ID MoreIds;") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(MOREIDS);
                    ss.push(ID_NT);
                    ss.push(TYPE);
                }

                Some(17) => {
                    if let Err(e) = writeln!(file,"Rule: Type⟶ int") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(INT);
                }

                Some(18) => {
                    if let Err(e) = writeln!(file,"Rule: Type⟶ bool") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(BOOL);
                }

                Some(19) => {
                    if let Err(e) = writeln!(file,"Rule: Type⟶ float") {
                eprintln!("Could not write to file: {}", e);       
                    }             
                    ss.pop();
                    ss.push(FLOAT);
                }

                //SEMI COLON
                /*Some(20) => {
                    ss.pop();
                    ss.push(STATEMENT);
                    ss.push(SEMICOLON);
                }

                Some(21) => {
                    ss.push(STATEMENT);
                    ss.push(SEMICOLON);
                }*/
                //Handles IF statements
                Some(23) => {
                    if let Err(e) = writeln!(file,"Rule: Statement⟶if Conditional then Statement else Statement endif") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.push(ENDIF);
                    ss.push(STATEMENT);
                    ss.push(ELSE);
                    ss.push(STATEMENT);
                    ss.push(THEN);
                    ss.push(CONDITIONAL);
                    ss.push(IF);
                }

                //Handles WHILE statements
                Some(24) => {
                    if let Err(e) = writeln!(file,"Rule: Statement⟶ while Conditional do Statement whileend ") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(WHILEEND);
                    ss.push(SEMICOLON);
                    ss.push(STATEMENT);
                    ss.push(DO);
                    ss.push(CONDITIONAL);
                    ss.push(WHILE);
                }

                //Handles begin statements
                Some(25) => {
                    if let Err(e) = writeln!(file,"Rule Statement ⟶  begin Statement MoreStatements end") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(END);
                    ss.push(MORESTATEMENTS);
                    ss.push(STATEMENT);
                    ss.push(BEGIN);
                }

                //Handles morestatements
                Some(26) => {
                    if let Err(e) = writeln!(file,"Rule: MoreStatements⟶ ; Statement MoreStatements") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(MORESTATEMENTS);
                    ss.push(STATEMENT);
                    ss.push(SEMICOLON);
                }

                //Handle conditionals
                Some(27) => {
                    let mut temp_pointer = token_pointer;
                    let mut relop_check = false;
                    //Check for a Relop
                    while temp_pointer < token_list.len() {
                        match lexer_to_symbol(&token_list[temp_pointer]) {
                            LTHAN => {
                                relop_check = true;
                                break;
                            }

                            LEQUAL => {
                                relop_check = true;
                                break;
                            }

                            EQUALTO => {
                                relop_check = true;
                                break;
                            }

                            NOTEQUAL => {
                                relop_check = true;
                                break;
                            }

                            GTHANEQUAL => {
                                relop_check = true;
                                break;
                            }

                            GTHAN => {
                                relop_check = true;
                                break;
                            }
                            THEN => break,

                            _ => temp_pointer = temp_pointer + 1,
                        }
                    }

                    if relop_check {
                        if let Err(e) = writeln!(file,"Rule:  Conditional⟶ Expression Relop Expression") {
                eprintln!("Could not write to file: {}", e);
                        }
                        ss.pop();
                        ss.push(EXPR);
                        ss.push(RELOP);
                        ss.push(EXPR);
                    } else {
                        if let Err(e) = writeln!(file,"Rule: Conditional⟶ Expression") {
                eprintln!("Could not write to file: {}", e);
                        }
                        println!();
                        ss.pop();
                        ss.push(EXPR);
                    }
                }

                Some(28) => {
                    if let Err(e) = writeln!(file,"Rule: Relop⟶ <") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(LTHAN);
                }

                Some(29) => {
                    if let Err(e) = writeln!(file,"Rule: Relop⟶ <=") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(LEQUAL);
                }

                Some(30) => {
                    if let Err(e) = writeln!(file,"Rule: Relop⟶ ==") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(EQUALTO);
                }

                Some(31) => {
                    if let Err(e) = writeln!(file,"Rule: Relop⟶ <>") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(NOTEQUAL);
                }
                Some(32) => {
                    if let Err(e) = writeln!(file,"Rule: Relop⟶ >=") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(GTHANEQUAL);
                }

                Some(33) => {
                    if let Err(e) = writeln!(file,"Rule: Relop⟶ >") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.pop();
                    ss.push(GTHAN);
                }

                Some(36) => {
                    if let Err(e) = writeln!(file,"Rule: MoreIds⟶ , ID MoreIds") {
                eprintln!("Could not write to file: {}", e);
                    }
                    ss.push(ID_NT);
                    ss.push(COMMA);
                    symbol_flag = true;
                }

                //default
                _ => {
                    println!(
                        "ERROR! Expected {:?} but got {:?} on line {:?}",
                        ss[ss.len() - 1],
                        lexer_to_symbol(&token_list[token_pointer]),
                        token_list[token_pointer].line
                    );
                    break;
                }
            }
        }
    }

    // Get output path.
    // Hacky way to print out the input ui.
    // Passing a mutable 1 will trigger the second condition.

    // TODO Output to file using output path. Maybe have print_symbol_table output straight to the output path.
    print_symbol_table(ST, output_path);
}

fn return_enum_string(temp: Symbols) -> String {
    match temp {
        INT => "Integer".to_string(),
        BOOl => "Bool".to_string(),
        FLOAT => "Float".to_string(),
        _ => "Ooops".to_string(),
    }
}

fn print_symbol_table(ST: Vec<(String, String, usize)>, output_path: String) {
    let mut table = Table::new();

    //add header
    table.add_row(row!["TYPE", "Variable", "Line#"]);

    //add data
    for i in ST {
        table.add_row(row![i.0, i.1, i.2]);
    }
    let mut output_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path.trim())
        .unwrap();

    //print table to stdout
    match table.print(&mut output_file) {
        Ok(num) => println!("Successfully printed to {:?}", output_file),
        Err(err) => println!("{}", err),
    }
}
