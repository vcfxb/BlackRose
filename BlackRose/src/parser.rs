use lexer::LexedLine;
//TODO: Parser (Both single and multi-line)
pub fn parse_line(lexed_line: LexedLine) -> ParsedLine {
    let ret_stmnt: Stmnt;
    if lexed_line.line.is_empty() {
        ret_stmnt = Stmnt::None;
    } else {
        let mut word_stack = vec![];
        for word in lexed_line.line {
            word_stack.push(word.into_iter().collect());
        }
        ret_stmnt = parse_vec(word_stack);
    }
    return ParsedLine{ original_line: lexed_line.original_line, line_num:lexed_line.line_num, statement: ret_stmnt }
}

fn parse_vec<'c>(line: Vec<Vec<char>>) -> Stmnt<'c> {
    let mut index: usize = 0;
    let mut ret_statement : Stmnt = Stmnt::None;
    loop {
        match line[index] {
            _ => {
                break;
            },
        }
    }
    return ret_statement;
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

pub enum Type { //tuple typing
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    Int,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    String,
    SetArray(Box<Type>), // Array of set size
    Array(Box<Type>),  // contains a single type
    List,       // can contain many of any type?
}

pub enum Stmnt<'a> {
    Vardec {
        id: Expr<'a>,
        given_type: Vec<Type>,
    },
    Assign {
        id: Expr<'a>,
        val: Expr<'a>,
    },
    VarDecAssign {
        id: Expr<'a>,
        given_type: Vec<Type>,
        val: Expr<'a>,
    },
    Expr {
      expression: Expr<'a>,
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


