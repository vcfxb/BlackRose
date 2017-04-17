use preproc::UTF8Line as ULine;
pub fn lex_lines<'a>(p_line_list: Vec<ULine<'a>>) -> Vec<LexedLine> {
    let mut final_list: Vec<LexedLine> = vec![];
    for line_listing in p_line_list {
        final_list.push(LexedLine{ line_num: line_listing.line_num, line: lex_line(line_listing.line), original_line: line_listing.original_line });
    }
    return final_list;
}

pub fn lex_line(cl: Vec<u8>) -> Vec<Vec<char>> {
    let mut final_list: Vec<Vec<char>> = vec![];
    let mut char_list: Vec<char> = vec![];
    for c in cl {
        char_list.push(c as char);              // change Vec<u8> to Vec<char>
    }
    char_list.reverse();                       // make it so that char_list.pop() actually returns the first rather than the last
    loop {
        let mut work_list: Vec<char> = vec![];
        let character: char;
        if let Some(n) = char_list.pop() {     //if let statement is easier/better than match statement
            character = n;
        }
        else {
            break;          // stop at end of line
        };
        // go on to character matching
        if character == '\"' { // Past here is essentially a match statement, but with more power to it
            work_list.push(character);
            let mut continue_loop = true;
            while continue_loop == true {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n == '\"' {
                            work_list.push(n);
                            continue_loop = false;
                        } else {
                            work_list.push(n);
                        }
                    },
                    None => break,
                };
            }
        }
        else if character == '\'' { // Past here is essentially a match statement, but with more power to it
            work_list.push(character);
            let mut continue_loop = true;
            while continue_loop == true {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n == '\'' {
                            work_list.push(n);
                            continue_loop = false;
                        } else {
                            work_list.push(n);
                        }
                    },
                    None => break,
                };
            }
        }   // Quotes first
        else if [' ', '\n'].contains(&character) {}
        else if character.is_numeric() {      //is_numeric hopefully doesn't let in letters
            work_list.push(character);
            let mut continue_loop = true;
            let mut used_decimal = false;
            while continue_loop == true {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n.is_numeric() {
                            work_list.push(n);
                        } else if n == '.' {
                            if !used_decimal {
                                work_list.push('.');
                            }
                            else {
                                continue_loop = false;
                                char_list.push('.');
                            }
                        } else {
                            char_list.push(n);     // if not one of the desired characters here, put it back
                            continue_loop = false;
                        }
                    },
                    None => break,
                };
            }
            if work_list == ['0'] {
                let mut binary = false;
                let mut hex = false;
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if ['x','b'].contains(&n) { // x: hexadecimal, b: binary
                            work_list.push(n);
                            continue_loop = true;
                            if n == 'x' {
                                hex = true;
                                binary = false;
                            } else if n == 'b' {
                                hex = false;
                                binary = true;
                            }
                        } else {
                            char_list.push(n);     // if not one of the desired characters here, put it back
                            continue_loop = false;
                        }
                    },
                    None => break,
                };
                while continue_loop == true {
                    match char_list.pop() {     // stop at end of line
                        Some(n) => {
                            if (['A','a','b','B','C','c','d','D','E','e','f','F'].contains(&n)) | (n.is_numeric()) {
                                if binary {
                                    if (n == '1') | (n == '0') {
                                        work_list.push(n);
                                    } else {
                                        char_list.push(n);
                                        continue_loop = false;
                                    }
                                } else if hex {
                                    work_list.push(n);
                                } else {
                                    panic!("Number Doesn't Make Sense!");       // In theory don't put panics in, but here, its fine.
                                }
                            } else if n == '.' {
                                if !used_decimal {
                                    work_list.push('.');
                                }
                                else {
                                    continue_loop = false;
                                    char_list.push('.');
                                }
                            } else {
                                char_list.push(n);     // if not one of the desired characters here, put it back
                                continue_loop = false;
                            }
                        },
                        None => break,
                    };
                }
            } //specialnumbers
        }
        else if character.is_alphanumeric() {
            work_list.push(character);
            let mut continue_loop = true;
            while continue_loop == true {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n.is_alphanumeric() && !(n.is_numeric()) {  // if n is an alphabetical character
                            work_list.push(n);
                        } else {
                            char_list.push(n);     // if not one of the desired characters here, put it back
                            continue_loop = false;
                        }
                    },
                    None => break,
                };
            }
        }
        else if ['(', ')', '.', ';', '{', '}', ','].contains(&character) {
            work_list.push(character);
        }
        // Special characters
        else if character == '=' {
            work_list.push('=');
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if n == '=' {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else if character == ':' {
            work_list.push(':');
            match char_list.pop() {
                Some(n) => {
                    if n == ':' {
                        work_list.push(':');
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            }
        }
        else if character == '-' {
            work_list.push('-');        // aka the character
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if ['=', '>', '-'].contains(&n) {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else if character == '+' {
            work_list.push('+');
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if ['=', '+'].contains(&n) {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else if character == '!' {
            work_list.push('!');
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if n == '=' {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else if character == '<' {
            work_list.push('<');
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if (n == '=') | (n == '-') {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else if character == '>' {
            work_list.push('>');
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if n == '=' {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else if character == '*' {
            work_list.push('*');
            match char_list.pop() {     // stop at end of line
                Some(n) => {
                    if n == '=' {
                        work_list.push(n);
                    } else {
                        char_list.push(n);     // if not one of the desired characters here, put it back
                    }
                },
                None => break,
            };
        }
        else {
            work_list.push(character);
        }
        if !(work_list.is_empty()) {        // only push if not empty
            final_list.push(work_list);
        }
    }
    return final_list;
}

pub struct LexedLine<'b> {
    pub line_num: usize,
    pub line: Vec<Vec<char>>,
    pub original_line: &'b str,
}