use std::process;

fn get_err_sig(error: &str) -> (i32, i32) {
    match error {               // this is essentially a Hash table?
        "NoError" => (0,1),
        "ArgumentError" => (1,2),
        "InvalidCharacterError" => (1,3),
        "WriteOutError" => (1,4),
        _ => (1,0),
    }
}
pub struct Error<'a> {
    pub error_type: &'a str,
    pub line_num: usize,
    pub loc: usize,
    pub line: &'a str,
    pub filename: &'a str,
    pub info: &'a str,
}

pub fn execute(err: Error) {
    let mut interactive = false;
    let mut finalvec = vec![];
    if err.filename == "" {
        interactive = true;
    }
    if err.line_num == 0 {
        finalvec.push("\n".to_string()+err.error_type+" at command prompt:\n")
    } else if err.loc == 0 {
        let s = err.line_num.to_string();
        let ss: &str = &s;
        finalvec.push(err.error_type.to_string()+" at line "+ss+" in file "+err.filename+":")
    } else {
        let mut s = err.line_num.to_string();
        let l = err.loc.to_string();
        let ll :&str = &l;
        s.push(':');
        s = s+ ll;
        if interactive == true {
            finalvec.push(err.error_type.to_string()+" at "+&s+" during interactive session:");
        } else {
            finalvec.push(err.error_type.to_string()+" at "+&s+" in file "+err.filename+":");
        }
    }
    {
        let templine = "  ".to_string()+err.line;
        let t: &str = &templine;
        finalvec.push(t.to_string());
    }
    if err.loc == 0 {
        let mut templine = "  ".to_string();
        templine = templine + ("-".repeat(err.line.len())).as_str()+"\n";
        finalvec.push(templine)
    } else {
        let mut templine = "  ".to_string();
        templine = templine + ("-".repeat(err.loc-1)).as_str()+"^\n";
        finalvec.push(templine);
    }
    finalvec.push(err.info.to_string());
    println!("{}", finalvec.join("\n"));
    if interactive == false {
        process::exit(get_err_sig(err.error_type).0);
    } else {
        println!("Error Signature: {}", get_err_sig(err.error_type).1);
    }
}
