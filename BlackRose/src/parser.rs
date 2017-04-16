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

pub fn parse_vec<'c>(line: Vec<String>) -> Stmnt<'c> {
    let mut index: usize = 0;
    let mut ret_statement : Stmnt = Stmnt::None;
    let mut parsed_list : Vec<PreAstTokens> = vec![];       //pre-AST representation
    while index <= line.len()-1 {
        match line[index].as_str() {
            "true" | "false" => {
                parsed_list.push(PreAstTokens::LitBool(line[index].parse::<bool>().unwrap()));      // Litbool check
            },
            "(" => {
                parsed_list.push(PreAstTokens::StartParen);
            },
            ")" => {
                parsed_list.push(PreAstTokens::EndParen);
            },
            "[" => {
                parsed_list.push(PreAstTokens::StartBracket);
            },
            "]" => {
                parsed_list.push(PreAstTokens::EndBracket);
            },
            "{" => {
                parsed_list.push(PreAstTokens::StartBrace);
            },
            "}" => {
                parsed_list.push(PreAstTokens::EndBrace);
            },
            "-" => {
                parsed_list.push(PreAstTokens::Minus);
            },
            "+" => {
                parsed_list.push(PreAstTokens::Plus);
            },
            "-=" => {
                parsed_list.push(PreAstTokens::MinusEquals);
            },
            "+=" => {
                parsed_list.push(PreAstTokens::PlusEquals);
            },
            "->" => {
                parsed_list.push(PreAstTokens::RightArrow);
            },
            "<-" => {
                parsed_list.push(PreAstTokens::LeftArrow);
            },
            "<=" => {
                parsed_list.push(PreAstTokens::LessThanOrEqual);
            },
            ">=" => {
                parsed_list.push(PreAstTokens::GreaterThanOrEqual);
            },
            ">" => {
                parsed_list.push(PreAstTokens::GreaterThan);
            },
            "<" => {
                parsed_list.push(PreAstTokens::LessThan);
            },
            "." => {
                parsed_list.push(PreAstTokens::Dot);
            },
            "," => {
                parsed_list.push(PreAstTokens::Comma);
            },
            "==" => {
                parsed_list.push(PreAstTokens::DoubleEquals);
            },
            "--" => {
                parsed_list.push(PreAstTokens::DoubleMinus);
            },
            "++" => {
                parsed_list.push(PreAstTokens::DoublePlus);
            },
            "!=" => {
                parsed_list.push(PreAstTokens::NotEqual);
            },
            "!" => {
                parsed_list.push(PreAstTokens::Not);
            },
            "*" => {
                parsed_list.push(PreAstTokens::Times);
            },
            "*=" => {
                parsed_list.push(PreAstTokens::TimesEquals);
            },
            ";" => {
                parsed_list.push(PreAstTokens::EndOfLine);
            },
            _ => {
                match line[index].parse::<usize>() {
                    Ok(n) => {
                        parsed_list.push(PreAstTokens::LitUnsignedInt(n));
                        index += 1;
                        continue;
                    },
                    Err(_) => {},
                }
                let mut char_vec : Vec<char> = line[index].chars().collect();
                if (char_vec[0] == '\'') & (char_vec.last().unwrap() == &'\'') & (char_vec.len() == 3) {
                    parsed_list.push(PreAstTokens::LitChar(char_vec[1]));
                }
                else if (char_vec[0] == '\'') & (char_vec.last().unwrap() == &'\'') & !(char_vec.len() == 3) {
                    char_vec.pop();
                    char_vec.reverse();
                    char_vec.pop();
                    char_vec.reverse();
                    parsed_list.push(PreAstTokens::LitCharErr(char_vec.into_iter().collect()));
                }
                else if (char_vec[0] == '\"') & (char_vec.last().unwrap() == &'\"') { //Lit Strings
                    char_vec.pop();
                    char_vec.reverse();
                    char_vec.pop();
                    char_vec.reverse();
                    parsed_list.push(PreAstTokens::LitString(char_vec.into_iter().collect()));
                }
                else {
                    let mut is_num = true;

                }
            },
        }
        index += 2;
    }
    // todo: Generate AST from vec
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
    Num,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    Char,
    F64,
    String,
    BigInt(BigInt),
    BigFloat(BigFloat),
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
    LitUnsignedInt(usize),
    LitSignedInt(isize),
    LitFloat64(f64),
    LitFloat32(f32),
    LitBigInt(BigInt),
    LitBigFloat(BigFloat),
    LitString(&'b str),
    LitChar(char),
    Neg(Box<Expr<'b>>),
    Not(Box<Expr<'b>>),
    Factorial(Box<Expr<'b>>),
    Add(Box<Expr<'b>>, Box<Expr<'b>>),
    Subtract(Box<Expr<'b>>, Box<Expr<'b>>),
    Multiply(Box<Expr<'b>>, Box<Expr<'b>>),
    Divide(Box<Expr<'b>>, Box<Expr<'b>>),
    Modulo(Box<Expr<'b>>, Box<Expr<'b>>),
    Expr(Box<Expr<'b>>),
    EndOfLine,
}

pub struct BigInt {
    base: u8,
    num: Vec<u8>,
    neg: bool,
}

pub struct BigFloat {
    base: u8,
    neg: bool,
    num: Vec<u8>,
    decimal_location: usize,        // after which index is the decimal point located?
}

enum PreAstTokens {     //Lexer Tokens
    StartParen,
    EndParen,
    StartBrace,
    EndBrace,
    StartBracket,
    EndBracket,
    Plus,
    Minus,
    Equals,
    Dot,
    Comma,
    DoubleEquals,
    DoubleColon,
    MinusEquals,
    DoubleMinus,
    RightArrow,
    LeftArrow,
    PlusEquals,
    DoublePlus,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LessThan,
    GreaterThan,
    Not,
    Times,
    TimesEquals,
    EndOfLine,
    LitBool(bool),
    LitUnsignedInt(usize),
    LitFloat64(f64),
    LitFloat32(f32),
    LitString(String),
    LitChar(char),
    LitCharErr(String),
    LitBigInt(BigInt),
    LitBigFloat(BigFloat),
}