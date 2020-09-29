 #[path = "../lexer/fsm.rs"]
 pub mod fsm;
 pub use fsm::FsmTransitions;
 pub use fsm::FsmTransitions::*;

#[derive(Clone)]
pub struct TokenType {
    pub token: String,
    pub lexeme: fsm::FsmTransitions,
    pub lexeme_name: String,
}

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType {
            token: "".to_string(),
            lexeme: fsm::_Reject,
            lexeme_name: "".to_string(),
        }
    }
}