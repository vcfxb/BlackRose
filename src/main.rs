extern crate blackrose;
use blackrose::blackroseerrors as errors;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let arglenstring = (args.len()-1).to_string();
        let error_info = "Expected 1 argument, received ".to_string()+&arglenstring+" arguments.";
        errors::execute(errors::Error{error_type: "ArgumentError", linenum: 0, filename:"command", loc: 0, line: &args.join(" "), info: &error_info });
    } else if args.len() == 2 {
        let mut file = match File::open(&args[1]) {
            Ok(file) => file,
            Err(e) => {
                let error_info_orig = &args[1];
                let error_info = error_info_orig.to_string() + " could not be opened as a valid file!";
                errors::execute(errors::Error{error_type: "ArgumentError", linenum: 0, filename:&args[1], loc: 0, line: &args.join(" "), info: error_info.as_str()});
                panic!(e);
            },
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(a) => a,
            Err(e) => {
                let error_info_orig = &args[1];
                let error_info = error_info_orig.to_string() + " could not be read as a valid file!";
                errors::execute(errors::Error{error_type: "ArgumentError", linenum: 0, filename:&args[1], loc: 0, line: &args.join(" "), info: error_info.as_str()});
                panic!(e);
            },
        };
        run_file(contents, &args[1]);
    } else {
        let prompt = vec!["radon"];
        run_prompt(&prompt);
    }
}

fn run_file(s :String, filename :&str) {
    println!("{}", s);
}

fn run_prompt(inlist :&[&str]) {
    let mut current_line :usize = 1;
    loop {
        print!("{}:{} > ", inlist.join(":"), current_line);
        match io::stdout().flush(){
            Ok(a) => a,
            Err(e) => {
                let error_info = "Could not write to standard output! (Fatal)";
                errors::execute(errors::Error{error_type: "WriteOutError", linenum: current_line, filename:" ", loc: 0, line:"<interactive prompt>", info: error_info});
                panic!(e);
            },
        };
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(a) => a,
            Err(e) => {
                errors::execute(errors::Error{error_type: "InvalidCharacterError", linenum: current_line, filename:"", loc: 0, line: &buffer.as_str(), info: (buffer.as_str().to_string()+ " contained invalid charcters!").as_str()});
                panic!(e);
            },
        };
        println!("{}", buffer);
        match io::stdout().flush() {
            Ok(a) => a,
            Err(e) => {
                let error_info = "Could not write to standard output! (Fatal)";
                errors::execute(errors::Error{error_type: "WriteOutError", linenum: current_line, filename:" ", loc: 0, line: buffer.as_str(), info: error_info});
                panic!(e);
            },
        };
        current_line = current_line + 1;
    }
}

// def run_prompt(prompt):
//     while True:
//         try:
//             t = input(':'.join(prompt))
//             print(lex(preproc([t])))
//         except KeyboardInterrupt:
//             print('\nexit')
//             sys.exit(0)
//
// main(sys.argv)
