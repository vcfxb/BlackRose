fn get_err_sig(error: &str) -> (u8, u8) {
    match error {
        "NoError" => (0,1),
        "ArgumentError" => (1,2),
        _ => (1,0),
    }
}
pub struct Error<'a> {
    pub error_type: &'a str,
    pub linenum: u64,
    pub loc: u64,
    pub line: &'a str,
    pub filename: &'a str,
    pub expectedargs: u8,
    pub receivedargs: usize,
}

pub fn execute(err: Error) {
    let mut finalvec = vec![];
    if err.linenum == 0 {
        finalvec.push(err.error_type.to_string()+" at command prompt:")
    } else if err.loc == 0 {
        let s = err.linenum.to_string();
        let ss: &str = &s;
        finalvec.push(err.error_type.to_string()+" at line "+ss+" in file "+err.filename+":")
    } else {
        let mut s = err.linenum.to_string();
        let l = err.loc.to_string();
        let ll :&str = &l;
        s.push(':');
        s = s+ ll;
        finalvec.push(err.error_type.to_string()+" at "+&s+" in file "+err.filename+":")
    }
    println!("{}", finalvec.join("\n"));
}

// Old Python implementation
//
// def execute(ERR):
//     final = []
//     final.append(ERR.TYPE+" at "+ERR.location+" :\n")
//     final.append("  "+ERR.line)
//     if ERR.location == 'command prompt':
//         final.append('  '+('-'*len(ERR.line)))
//     else:
//         final.append('  '+('-'*(int(ERR.location.split(':')[1])-1)+'^'))
//     final.append('\n'+ERR.info)
//     print('\n'.join(final))
//     sys.exit(errors[ERR.TYPE][0])
