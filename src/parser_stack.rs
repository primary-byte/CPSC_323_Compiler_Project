use crate::file_handling::lexer::*;
use std::fs::OpenOptions;
use std::io::Write; //string operations

//derive operations to perform deep copies of the enum later
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Symbols{
    //Terminals
    L_PAREN,        // (
    R_PAREN,        // )
    PLUS,           // +
    MINUS,          // -
    MULT,           // *
    DIV,            // /
    ID,             // ID
    END_OF_STACK,   // $
    
    //Non-Terminals
    EXPR,           // TE'
    EXPR_PRIME,     // +TE' | -TE' | EPSILON
    TERM,           // F | T'
    TERM_PRIME,     // *FT' | /FT'
    FACTOR          // (E) | ID | <NUM>

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
1	−	−	−	−	+	−	+	+	−
2	+	−	−	−	−	−	−	−	−
3	−	+	−	−	−	−	−	−	−
4	−	−	−	−	−	+	−	−	+
5	−	−	−	−	+	−	+	+	−
6	−	−	+	−	−	−	−	−	−
7	−	−	−	+	−	−	−	−	−
8	+	+	−	−	−	+	−	−	+
9	−	−	−	−	+	−	−	−	−
10	−	−	−	−	−	−	−	+	−
11	−	−	−	−	−	−	+	−	−
12	−	−	−	−	−	−	−	+	−






*/


pub const LL_TABLE: &[&[]] = &[
    &[

    ],

]