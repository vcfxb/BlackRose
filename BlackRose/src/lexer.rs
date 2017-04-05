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
        let character = match char_list.pop() {     // stop at end of line
            Some(n) => n,
            None => break,
        };
        if character == '\"' { // Past here is essentially a match statement, but with more power to it
            work_list.push(character);
            loop {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n == '\"' {
                            work_list.push(n);
                            break;
                        } else {
                            work_list.push(n);
                        }
                    },
                    None => break,
                };
            }
        }
        else if character == ' ' {}
        else if character.is_numeric() {      //is_numeric hopefully doesnt let in letters
            work_list.push(character);
            loop {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n.is_numeric() {
                            work_list.push(n);
                        } else if n == '.' {
                            work_list.push(n);
                        } else {
                            char_list.push(n);     // if not one of the desired characters here, put it back
                            break;
                        }
                    },
                    None => break,
                };
            }
        }
        else if character.is_alphanumeric() {
            work_list.push(character);
            loop {
                match char_list.pop() {     // stop at end of line
                    Some(n) => {
                        if n.is_alphanumeric() {
                            work_list.push(n);
                        } else {
                            char_list.push(n);     // if not one of the desired characters here, put it back
                            break;
                        }
                    },
                    None => break,
                };
            }
        }
        else if ['(', ')', '.', ';', '{', '}', ':'].contains(&character) {
            work_list.push(character);
        } else {
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


// def oldlex(linelist):
//     final = []
//     for x, line in enumerate(linelist):
//         temp = ''
//         stemp = []
//         fmode = False
//         smode = False
//         for y, char in enumerate(line):
//             if smode == True:
//                 if char in ["'", '"']:
//                     stemp.append(char)
//                     smode = False
//                     final.append(''.join(stemp))
//                     stemp = []
//                     continue
//                 else:
//                     stemp.append(char)
//                     continue
//             elif smode == False:
//                 if char in ["'", '"']:
//                     stemp.append(char)
//                     smode = True
//                     continue
//                 else:
//                     pass
//             if 48 <= ord(char) <= 57:
//                 temp = temp + char
//                 if 48 <= ord(line[y+1]) <= 57:
//                     continue
//                     continue
//                 else:
//                     if fmode == False:
//                         continue
//                 else:
//                     if fmode == False:
//                         final.append(int(temp))
//                         temp = ''
//                     elif fmode == True:
//                         final.append(float(temp))
//                         temp = ''
//                         fmode = False
//             elif char == '.':
//                 if (48 <= ord(line[y-1]) <= 57) & (48 <= ord(line[y+1]) <= 57):
//                     temp = temp+char
//                 else:
//                     final.append(char)
//             elif (0x41 <= ord(char) <= 0x5A) | (0x61 <= ord(char) <= 0x7A):
//                 temp = temp+char
//                 if (0x41 <= ord(line[y+1]) <= 0x5A) | (0x61 <= ord(line[y+1]) <= 0x7A):
//                     continue
//                 else:
//                     if temp == 'True':
//                         final.append(True)
//                     elif temp == 'False':
//                         final.append(False)
//                     else:
//                         final.append(temp)
//                     temp = ''
//             elif char == ' ':
//                 continue
//             elif char == '\n':
//                 continue
//             else:
//                 final.append(char)
//     return final
//
// def lex(enumeratedlines):
//     worklist = []
//     final = []
//     for pair in enumeratedlines:
//         for x in pair[1].split(';'):
//             worklist.append([pair[0], x])
//     for n, pair in enumerate(worklist):
//         if pair[1] in ['\n','']:
//             worklist[n] = None
//     for x in range(worklist.count(None)):
//         del worklist[worklist.index(None)]
//     for
//     final = worklist
//     return worklist
