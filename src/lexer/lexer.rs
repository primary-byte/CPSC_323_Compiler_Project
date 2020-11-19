#[path = "../tokens/tokens.rs"]
pub mod tokens;
pub use tokens::*;

//parse a given string into a vector of tokens via use of FSM
// to modify change "fsm.rs"
pub fn lexer(expression: &String) -> Vec<TokenType> {
    let mut access = TokenType::default();          
    let mut tokens: Vec<TokenType> = Vec::new();    //stores tokens previously collected
    let mut col: FsmTransitions;                    //current column of Table
    let mut prev_state: FsmTransitions = _Reject;   //Previous state we were in
    let mut current_state: FsmTransitions = _Reject;//Current state we are in
    let mut current_token = String::new();          //current token we are working on
    let mut index = 0;
    
    //loop through characters
    while index != expression.len() {
        col = get_col(match expression.chars().nth(index) {
            Some(c) => c,
            None => ' ',
        });

        //Debug code here
        // println!("Collum is {} .", col as i32);
        current_state = fsm::STATE_TABLE[current_state as usize][col as usize];

        //for rejection state
        if current_state == _Reject {
            //check precursor states for validity, if not found then continue to next
            if prev_state != _Space {
                if prev_state == _Comment {
                    current_token.push(match expression.chars().nth(index) {
                        Some(c) => c,
                        None => ' ',
                    });

                    index = index + 1;
                }

                //check if string is identifier or keyword

                if prev_state == _String {
                    prev_state = get_string_type(current_token.clone());
                }

                //add the valid token
                access.token = current_token;
                access.lexeme = prev_state;
                access.lexeme_name = get_lexeme_name( &access.lexeme );
                tokens.push(access.clone());
            }

            current_token = "".to_string();
        } else {
            current_token.push(match expression.chars().nth(index) {
                Some(c) => c,
                None => ' ',
            });

            index = index + 1;
        }

        prev_state = current_state;
    }
    if current_state == _String {
                    current_state = get_string_type(current_token.clone());
                }
                
    if current_state != _Space && current_token != "" {
        access.token = current_token.clone();
        access.lexeme = current_state;
        access.lexeme_name = get_lexeme_name(&access.lexeme);
        tokens.push(access.clone());
    }

    tokens
}

//use our state table and enums with match to make pretty things out of ugly code
fn get_col(c: char) -> FsmTransitions {
    if c.is_digit(10) {
        _Integer
    } else if c.is_alphabetic() {
        _String
    } else {
        match c {
            '!' => _Comment,
            ' ' => _Space,
            '\n' => _Space,
            '\r' => _Space,
            '\t' => _Space,
            '$' => _String,
            '.' => _Real,
            '\'' => _Separator,
            '{' => _Separator,
            '}' => _Separator,
            '[' => _Separator,
            ']' => _Separator,
            '(' => _Separator,
            ')' => _Separator,
            ',' => _Separator,
            ':' => _Separator,
            ';' => _Separator,
            '*' => _Operator,
            '+' => _Operator,
            '-' => _Operator,
            '=' => _Operator,
            '/' => _Operator,
            '>' => _Operator,
            '<' => _Operator,
            '%' => _Operator,
            _ => _Unknown,
        }
    }
}

//use enums with match to make pretty things out of ugly code
fn get_lexeme_name(lexeme: &FsmTransitions) -> String {
    match lexeme {
        _Comment => "COMMENT".to_string(),
        _Space => "SPACE".to_string(),
        _Separator => "SEPARATOR".to_string(),
        _Integer => "INTEGER".to_string(),
        _Real => "REAL".to_string(),
        _String => "STRING".to_string(),
        _Operator => "OPERATOR".to_string(),
        _Unknown => "UNKNOWN".to_string(),
        _Keyword => "KEYWORD".to_string(),
        _Identifier => "IDENTIFIER".to_string(),
        _ => "ERROR".to_string(),
    }
}

//pick out keywords and return the correct transition to the FSM
fn get_string_type(token: String) -> FsmTransitions {
    let keyword_vec = vec![
        "int", "float", "bool", "true", "false", "if", "else", "then", "endif", "while",
        "whileend", "do", "doend", "for", "forend", "input", "output", "and", "or", "not",
    ];

    if keyword_vec.contains(&token.as_str()) {
        return _Keyword;
    } else {
        return _Identifier;
    };
}
