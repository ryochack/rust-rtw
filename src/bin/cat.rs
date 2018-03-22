use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Option {
    number_noblank: bool,
    show_ends: bool,
    number: bool,
    number_count: u32,
    squeeze_blank: bool,
    show_tabs: bool,
    show_nonprinting: bool,
}

fn version() {
    println!("rust cat version 0.0.1");
}

fn help() {
    println!("help contents is here.");
}

fn cat(reader: &mut Read, opt: &Option) {
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    print!("{}", contents);
}

fn main() {
    let mut files: Vec<String> = Vec::new();
    let mut opt = Option {
        number_noblank: false,
        show_ends: false,
        number: false,
        number_count: 0,
        squeeze_blank: false,
        show_tabs: false,
        show_nonprinting: false,
    };

    for v in env::args().skip(1) {
        match v.as_str() {
            "-A" | "--show-all" => {
                // equivalent to -vET
                opt.show_nonprinting = true;
                opt.show_ends = true;
                opt.show_tabs = true;
            }
            "-b" | "--number-nonblank" => {
                // number nonempty output lines, overrides -n
                opt.number_noblank = true;
            }
            "-e" => {
                // equivalent to -vE
                opt.show_nonprinting = true;
                opt.show_ends = true;
            }
            "-E" | "--show-ends" => {
                // display $ at end of each line
                opt.show_ends = true;
            }
            "-n" | "--number" => {
                // number all output lines
                opt.number = true;
            }
            "-s" | "--squeeze-blank" => {
                // suppress repeated empty output lines
                opt.squeeze_blank = true;
            }
            "-t" => {
                // equivalent to -vT
                opt.show_nonprinting = true;
                opt.show_tabs = true;
            }
            "-T" | "--show-tabs" => {
                // display TAB characters as ^I
                opt.show_tabs = true;
            }
            "-u" => (), // (ignored)
            "-v" | "--show-nonprinting" => {
                // "use ^ and M- notation, except for LFD and TAB
                opt.show_nonprinting = true;
            }
            "--help" => {
                // display this help and exit
                help();
                return;
            }
            "--version" => {
                // output version information and exit
                version();
                return;
            }
            _ => files.push(v),
        }
    }

    println!("{:?}", files);
    for f in files {
        let mut reader = if f == "-" {
            Box::new(io::stdin()) as Box<Read>
        } else {
            Box::new(File::open(&f).expect("file not found")) as Box<Read>
        };
        cat(&mut reader, &opt);
    }
}
