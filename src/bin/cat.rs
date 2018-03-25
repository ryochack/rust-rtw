use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

#[allow(dead_code)]
struct CmdOption {
    number_noblank: bool,
    show_ends: bool,
    number: bool,
    number_count: u32,
    squeeze_blank: bool,
    blank_count: u32,
    show_tabs: bool,
    show_nonprinting: bool,
}

fn version() {
    println!("rust cat version 0.0.1");
}

fn help() {
    println!("help contents is here.");
}

fn cat(reader: &mut BufRead, opt: &mut CmdOption) {
    let mut contents = String::new();
    loop {
        match reader.read_line(&mut contents) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let newline_pat: &[_] = &['\n'];
                contents = format!("{}", contents.trim_right_matches(newline_pat));
                if opt.squeeze_blank {
                    if contents.len() == 0 {
                        opt.blank_count += 1;
                        if opt.blank_count > 1 {
                            continue;
                        }
                    } else {
                        opt.blank_count = 0;
                    }
                }
                if opt.show_nonprinting {
                    let mut rep_line = String::new();
                    for b in contents.as_str().bytes() {
                        let c = match b {
                            // ascii control characters
                            9 => "\t".to_string(),
                            0...31 => format!("^{}", (b + 64) as char),
                            // ascii graphic characters
                            32...126 => format!("{}", (b) as char),
                            127 => "^?".to_string(),
                            // 128 + 32 .. 128 + 126
                            160...254 => format!("M-{}", (b - 128) as char),
                            v if v > 254 => "M-^?".to_string(),
                            v => format!("^{}", (v - 128 + 64) as char),
                        };
                        rep_line += c.as_str();
                    }
                    contents = rep_line;
                }
                if opt.show_tabs {
                    contents = contents.replace("\t", "^I");
                }
                if (opt.number_noblank && contents.len() > 0) || opt.number {
                    contents = format!("{:>6}\t{}", opt.number_count, contents);
                    opt.number_count += 1;
                }
                if opt.show_ends {
                    contents = format!("{}$", contents);
                }
                println!("{}", contents);
            }
            Err(err) => {
                print!("{}", err);
                return;
            }
        }
        contents.clear();
    }
}

fn main() {
    let mut files: Vec<String> = Vec::new();
    let mut opt = CmdOption {
        number_noblank: false,
        show_ends: false,
        number: false,
        number_count: 1,
        squeeze_blank: false,
        blank_count: 0,
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

    if opt.number_noblank {
        // number nonempty output lines, overrides -n
        opt.number = false;
    }

    if files.len() == 0 {
        files.push("-".to_string());
    }

    for f in files {
        let mut r = if f == "-" {
            Box::new(io::stdin()) as Box<Read>
        } else {
            Box::new(File::open(&f).expect("file not found")) as Box<Read>
        };
        let mut reader = BufReader::new(r);
        cat(&mut reader, &mut opt);
    }
}
