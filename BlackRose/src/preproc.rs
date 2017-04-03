pub fn preprocessor<'c>(file_contents: &'c String) -> Vec<UTF8Line> {
    let mut working_lines: Vec<UTF8Line> = vec![];
    let mut final_lines: Vec<UTF8Line> = vec![];
    let line_iter = file_contents.lines();
    let mut mline_comment = false;              // MultiLine comments
    let mut line_n = 1;                         // line number
    for line_content in line_iter {
        if line_content.contains("###") {
            let string_vec: Vec<&str> = line_content.split("###").collect();
            if (string_vec.len() % 2) == 1 {            // if there were comments both started and ended in this line (comment state is not different at line end)
                if mline_comment == true {
                    let mut swap = true;
                    for part in string_vec {
                        if swap == true {
                            swap = false;
                        } else {
                            if part.is_empty() {
                            } else {
                                working_lines.push(UTF8Line { line: part.to_string().into_bytes(), line_num: line_n });
                            }
                            swap = true;
                        }
                    }
                } else {
                    let mut swap = false;
                    for part in string_vec {
                        if swap == true {
                            swap = false;
                        } else {
                            if part.is_empty() {
                            } else {
                                working_lines.push(UTF8Line { line: part.to_string().into_bytes(), line_num: line_n });
                            }
                            swap = true;
                        }
                    }
                }
            } else {                        // comment state changes (there is something to do this at the end of the conditionals.)
                if mline_comment == true {
                    let mut swap = true;
                    for part in string_vec {
                        if swap == true {
                            swap = false;
                        } else {
                            if part.is_empty() {
                            } else {
                                working_lines.push(UTF8Line { line: part.to_string().into_bytes(), line_num: line_n });
                            }
                            swap = true;
                        }
                    }
                } else {
                    let mut swap = false;
                    for part in string_vec {
                        if swap == true {
                            swap = false;
                        } else {
                            if part.is_empty() {
                            } else {
                                working_lines.push(UTF8Line { line: part.to_string().into_bytes(), line_num: line_n });
                            }
                            swap = true;
                        }
                    }
                }
                mline_comment = !mline_comment;
            }
        } else {
            if mline_comment == false {
                working_lines.push(UTF8Line { line: line_content.to_string().into_bytes(), line_num: line_n });
            }
        }
        line_n += 1;
    }
    // multi line comments done, now go to single line comments
    for numbered_line in working_lines {
        if numbered_line.line.contains(&0x23) {
            final_lines.push(UTF8Line { line_num: numbered_line.line_num, line: until_comment(numbered_line.line, 0x23) });
        } else {
            final_lines.push(numbered_line);
        }
    }
    return final_lines;
}

pub fn interactive_preprocessor(buffered_line: &str, inline_num: usize) -> UTF8Line {
    let mut l: UTF8Line = UTF8Line { line: buffered_line.to_string().into_bytes(), line_num: inline_num };
    if l.line.contains(&0x23) {
        l.line = until_comment(l.line, 0x23);
    }
    return l;
}

pub struct UTF8Line {
    pub line_num: usize,
    pub line: Vec<u8>,          // UTF8Line.line is a vector of bytes (utf-8 encodings)
}

fn until_comment(list: Vec<u8>, item: u8) -> Vec<u8> { // Only works if item is definitely in list
    let mut fv :Vec<u8> = vec![];
    for i in list {
        if i == item {
            break;
        } else {
            fv.push(i);
        }
    }
    return fv;
}
