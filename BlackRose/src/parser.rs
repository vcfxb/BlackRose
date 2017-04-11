use lexer::LexedLine;
//TODO: Parser (Both single and multi-line)
pub fn parse_line(lexed_line: LexedLine) -> ParsedLine {
    let ret_stmnt: Stmnt;
    if lexed_line.line.is_empty() {
        ret_stmnt = Stmnt::None;
    } else {
        let mut word_stack = vec![];
        for word in lexed_line {
            word_stack.push(word.into_iter().collect());
        }
        word_stack.reverse(); // Set up stack
        loop {
            let word;
            if let Some(n) = word_stack.pop() {     //if let statement is easier/better than match statement
                word = n;
            }
            else {
                    break;          // stop at end of line
                };
            if word == "(" {
                let parens: usize = 1;
                while parens > 0 {
                    word_stack
                }
            }
            else {

            }
        }
    }
    return ParsedLine{ original_line: lexed_line.original_line, line_num:lexed_line.line_num, statement: ret_stmnt }
}

pub fn parse_lines(lexed_lines: Vec<LexedLine>) -> Vec<ParsedLine> {
    let mut final_v: Vec<ParsedLine> = vec![];
    for lexed_line in lexed_lines {
        final_v.push(parse_line(lexed_line));
    }
    return final_v;
}

pub struct ParsedLine<'a> {
    pub original_line: &'a str,
    pub line_num: usize,
    pub statement: Stmnt<'a>,
}

pub struct Type<'a> {
    pub type_vec: Vec<&'a str>,         // 'Tuple-Typing'
}

pub enum Stmnt<'a> {
    Vardec {
        id: Expr<'a>,
        given_type: Type<'a>,
    },
    Assign {
        id: Expr<'a>,
        val: Expr<'a>,
    },
    VarDecAssign {
        id: Expr<'a>,
        given_type: Type<'a>,
        val: Expr<'a>,
    },
    None,

}

pub enum Expr<'b> {
    Id(&'b str),
    LitBool(bool),
    LitSignedInt(isize),
    LitUnsignedInt(usize),
    LitString(&'b str),
    Neg(Box<Expr<'b>>),
    Not(Box<Expr<'b>>),
    Factorial(Box<Expr<'b>>),
    Add(Box<Expr<'b>>, Box<Expr<'b>>),
    Subtract(Box<Expr<'b>>, Box<Expr<'b>>),
    Multiply(Box<Expr<'b>>, Box<Expr<'b>>),
    Divide(Box<Expr<'b>>, Box<Expr<'b>>),
    Modulo(Box<Expr<'b>>, Box<Expr<'b>>),
    EndOfLine
}


