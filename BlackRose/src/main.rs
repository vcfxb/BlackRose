extern crate blackrose;
use blackrose::blackroseerrors as errors;
use blackrose::preproc;
use blackrose::lexer;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 4 {
        let arglenstring = (args.len()-1).to_string();
        let error_info = "Expected 0 or 3 arguments, received ".to_string()+&arglenstring+" arguments.";
        errors::execute(errors::Error{error_type: "ArgumentError", line_num: 0, filename:"command", loc: 0, line: &args.join(" "), info: &error_info });
    } else if args.len() == 4 {
        let mut file = match File::open(&args[1]) {
            Ok(file) => file,
            Err(e) => {
                let error_info_orig = &args[1];
                let error_info = error_info_orig.to_string() + " could not be opened as a valid file!";
                errors::execute(errors::Error{error_type: "ArgumentError", line_num: 0, filename:&args[1], loc: 0, line: &args.join(" "), info: error_info.as_str()});
                panic!(e);
            },
        };
        let mut out_file = match File::open(&args[2]) {
            Ok(out_file) => out_file,
            Err(e) => {
                let error_info_orig = &args[2];
                let error_info = error_info_orig.to_string() + " could not be opened as a valid file!";
                errors::execute(errors::Error{error_type: "ArgumentError", line_num: 0, filename:&args[2], loc: 0, line: &args.join(" "), info: error_info.as_str()});
                panic!(e);
            },
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(a) => a,
            Err(e) => {
                let error_info_orig = &args[1];
                let error_info = error_info_orig.to_string() + " could not be read as a valid file!";
                errors::execute(errors::Error{error_type: "ArgumentError", line_num: 0, filename:&args[1], loc: 0, line: &args.join(" "), info: error_info.as_str()});
                panic!(e);
            },
        };
        let optimized: u8 = args[3].parse().unwrap();
        run_file(contents, out_file, &args[1], optimized);
    } else if args.len() == 1 {
        let prompt = vec!["radon"];
        run_prompt(&prompt);
    } else {
        let arglenstring = (args.len()-1).to_string();
        let error_info = "Expected 0 or 3 arguments, received ".to_string()+&arglenstring+" arguments.";
        errors::execute(errors::Error{error_type: "ArgumentError", line_num: 0, filename:"command", loc: 0, line: &args.join(" "), info: &error_info });
    }
}

fn run_file(s: String, mut output: File, file_name: &str, optimize: u8) {
    let p: Vec<preproc::UTF8Line> = preproc::preprocessor(&s);
    let l: Vec<lexer::LexedLine> = lexer::lex_lines(p);
    for i in l {
        print!("{}:Lexed:    ", i.line_num);
        for u in i.line {
            print!("{:?} ", u);
        }
        println!();
        println!("{}:Original: {}", i.line_num ,i.original_line);
    }
}

fn run_prompt(inlist: &[&str]) {
    let mut current_line :usize = 1;
    // Todo: Interactive Lexer
    loop {
        print!("{}:{} : ", inlist.join(":"), current_line);
        match io::stdout().flush(){
            Ok(a) => a,
            Err(e) => {
                let error_info = "Could not write to standard output! (Fatal)";
                errors::execute(errors::Error{error_type: "WriteOutError", line_num: current_line, filename:" ", loc: 0, line:"<interactive prompt>", info: error_info});
                panic!(e);
            },
        };
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(a) => a,
            Err(e) => {
                errors::execute(errors::Error{error_type: "InvalidCharacterError", line_num: current_line, filename:"", loc: 0, line: &buffer.as_str(), info: (buffer.as_str().to_string()+ " contained invalid characters!").as_str()});
                panic!(e);
            },
        };
        let full_line = preproc::interactive_preprocessor(&buffer, current_line);
        let lexed_line = lexer::LexedLine{ line_num: full_line.line_num, line: lexer::lex_line(full_line.line), original_line: full_line.original_line };
        print!("{}:>>> ", lexed_line.line_num);
        for word in lexed_line.line {
            let w: String = word.into_iter().collect();
            print!("\"{}\", ", w);
        }
        println!();
        match io::stdout().flush() {
            Ok(a) => a,
            Err(e) => {
                let error_info = "Could not write to standard output! (Fatal)";
                errors::execute(errors::Error{error_type: "WriteOutError", line_num: current_line, filename:" ", loc: 0, line: buffer.as_str(), info: error_info});
                panic!(e);
            },
        };
        current_line += 1;
    }
}

// def run_prompt(prompt):
//     while True:
//         try:
//             t = input(':'.join(prompt))
//             print(lex_line(preproc([t])))
//         except KeyboardInterrupt:
//             print('\nexit')
//             sys.exit(0)
