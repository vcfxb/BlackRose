use lexer::LexedLine;
//TODO: Parser (Both single and multi-line)
pub fn parse_line(lexed_line: LexedLine) -> ParsedLine {
    return ParsedLine{original_line: lexed_line.original_line, line_num:lexed_line.line_num, statement: Stmnt::None }
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
    Neg(Box<Expr<'b>>),
    Add(Box<Expr<'b>>, Box<Expr<'b>>),
    Subtract(Box<Expr<'b>>, Box<Expr<'b>>)
}


