pub fn preprocessor<'c>(file_contents: &'c String) -> Vec<Line> {
    let mut working_lines: Vec<Line> = vec![];
    let mut final_lines: Vec<Line> = vec![];
    let line_iter = file_contents.lines();
    for (line_n, line_content) in  line_iter.enumerate() {
        working_lines.push(Line{line: line_content.to_string().into_bytes(), line_num: line_n+1})
    }
    let mut mline_comment = false;              // MultiLine comments
    for numbered_line in working_lines {
        if numbered_line.line.contains(&0x23) {
            final_lines.push(Line{ line_num: numbered_line.line_num, line: until_comment(numbered_line.line, 0x23 ) });
        } else {
            final_lines.push(numbered_line);
        }
    }
    return final_lines;
}

pub fn interactive_preprocessor(buffered_line: &str, inline_num: usize) -> Line{
    return Line{ line: buffered_line.to_string().into_bytes(), line_num: inline_num };
}

pub struct Line {
    pub line_num: usize,
    pub line: Vec<u8>,          // Line.line is a vector of bytes (utf-8 encodings)
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

// old Python file
// def preprocessor(stringlist):
//     # remove comments
//     workl = stringlist
//     linenum = 0
//     for n, line in enumerate(stringlist):
//         if '#' not in line:
//             workl[n] = (linenum, line)
//             linenum += 1
//         else:
//             workl[n] = (linenum, line[0:line.index('#')])
//             linenum += 1
//     return workl
