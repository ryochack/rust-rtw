extern crate rtw;
use rtw::cat;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;

fn main() {
    let si = io::stdin();
    let mut stdin_stream = BufReader::new(si.lock());
    let so = io::stdout();
    let mut stdout_stream = so.lock();
    let se = io::stderr();
    let mut stderr_stream = se.lock();

    let args: Vec<String> = env::args().skip(1).collect();
    let mut files: Vec<String>;

    let mut cat = cat::CatBuilder::new().build();
    match cat.parse(args.as_slice()) {
        Ok(f) => files = f,
        Err(s) => {
            writeln!(&mut stderr_stream, "{}", s).unwrap();
            process::exit(1);
        }
    }

    if files.len() == 0 {
        files.push("-".to_string());
    }

    for fname in files {
        let mut file;
        let mut freader;
        let mut bufin_stream: &mut BufRead = if fname == "-" {
            // Box::new(io::stdin()) as Box<Read>
            &mut stdin_stream
        } else {
            // Box::new(File::open(&f).expect("file not found")) as Box<Read>
            file = File::open(&fname).expect(&format!("cat: {}: No such file or directory", fname));
            freader = BufReader::new(file);
            &mut freader
        };
        if cat.run(&mut bufin_stream, &mut stdout_stream, &mut stderr_stream)
            .is_err()
        {
            process::exit(1);
        }
    }
}
