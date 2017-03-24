pub fn preprocessor<'c>(file_contents: &'c String) -> Vec<Line<'c>> {
    let mut working_lines: Vec<Line> = vec![];
    let mut final_lines: Vec<Line> = vec![];
    let line_iter = file_contents.lines();
    for (line_n, line_content) in  line_iter.enumerate() {
        working_lines.push(Line{line: line_content, line_num: line_n+1})
    }
    let mut mline_comment = false;              // MultiLine comments
    for numbered_line in working_lines {
        if mline_comment {
            if numbered_line.line.contains("###") {
                // Find index of comment str
            }
        } else {
            if numbered_line.line.contains("###") {
                // See above
            } else {
                final_lines.push(numbered_line);
            }
        }
    }
    return final_lines;
}

pub fn interactive_preprocessor(buffered_line: &str, inline_num: usize) -> Line{
    return Line{ line: buffered_line, line_num: inline_num };
}

pub struct Line<'l> {
    pub line_num: usize,
    pub line: &'l str,
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
