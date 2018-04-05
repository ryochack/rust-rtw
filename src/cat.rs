#![allow(dead_code)]
use super::cliopt;
use std::io::prelude::*;

#[derive(Clone)]
struct CmdOption {
    number_noblank: bool,
    show_ends: bool,
    number: bool,
    squeeze_blank: bool,
    show_tabs: bool,
    show_nonprinting: bool,
    display_help: bool,
    display_version: bool,
}

pub struct CatBuilder {
    option: CmdOption,
}

pub struct Cat {
    option: CmdOption,
    number_count: u32,
    blank_count: u32,
}

impl CatBuilder {
    pub fn new() -> CatBuilder {
        CatBuilder {
            option: CmdOption {
                number_noblank: false,
                show_ends: false,
                number: false,
                squeeze_blank: false,
                show_tabs: false,
                show_nonprinting: false,
                display_help: false,
                display_version: false,
            },
        }
    }
    pub fn with_show_all(&mut self) -> &mut Self {
        self.option.show_nonprinting = true;
        self.option.show_ends = true;
        self.option.show_tabs = true;
        self
    }
    pub fn with_number_nonblank(&mut self) -> &mut Self {
        self.option.number_noblank = true;
        self
    }
    pub fn with_e(&mut self) -> &mut Self {
        self.option.show_nonprinting = true;
        self.option.show_ends = true;
        self
    }
    pub fn with_show_ends(&mut self) -> &mut Self {
        self.option.show_ends = true;
        self
    }
    pub fn with_number(&mut self) -> &mut Self {
        self.option.number = true;
        self
    }
    pub fn with_squeeze_blank(&mut self) -> &mut Self {
        self.option.squeeze_blank = true;
        self
    }
    pub fn with_t(&mut self) -> &mut Self {
        self.option.show_nonprinting = true;
        self.option.show_tabs = true;
        self
    }
    pub fn with_show_tabs(&mut self) -> &mut Self {
        self.option.show_tabs = true;
        self
    }
    pub fn with_show_nonprinting(&mut self) -> &mut Self {
        self.option.show_nonprinting = true;
        self
    }
    pub fn with_display_help(&mut self) -> &mut Self {
        if !self.option.display_version {
            self.option.display_help = true;
        }
        self
    }
    pub fn with_display_version(&mut self) -> &mut Self {
        if !self.option.display_help {
            self.option.display_version = true;
        }
        self
    }
    pub fn build(&self) -> Cat {
        Cat {
            option: self.option.clone(),
            number_count: 1,
            blank_count: 0,
        }
    }
}

impl Cat {
    fn parse_option(&mut self, opt: &str) -> Result<(), String> {
        match opt {
            "-A" | "--show-all" => {
                // equivalent to -vET
                self.option.show_nonprinting = true;
                self.option.show_ends = true;
                self.option.show_tabs = true;
            }
            "-b" | "--number-nonblank" => {
                // number nonempty output lines, overrides -n
                self.option.number_noblank = true;
            }
            "-e" => {
                // equivalent to -vE
                self.option.show_nonprinting = true;
                self.option.show_ends = true;
            }
            "-E" | "--show-ends" => {
                // display $ at end of each line
                self.option.show_ends = true;
            }
            "-n" | "--number" => {
                // number all output lines
                self.option.number = true;
            }
            "-s" | "--squeeze-blank" => {
                // suppress repeated empty output lines
                self.option.squeeze_blank = true;
            }
            "-t" => {
                // equivalent to -vT
                self.option.show_nonprinting = true;
                self.option.show_tabs = true;
            }
            "-T" | "--show-tabs" => {
                // display TAB characters as ^I
                self.option.show_tabs = true;
            }
            "-u" => (), // (ignored)
            "-v" | "--show-nonprinting" => {
                // "use ^ and M- notation, except for LFD and TAB
                self.option.show_nonprinting = true;
            }
            "--help" => {
                // display this help and exit
                if !self.option.display_version {
                    self.option.display_help = true;
                }
            }
            "--version" => {
                // output version information and exit
                if !self.option.display_help {
                    self.option.display_version = true;
                }
            }
            _ => {
                return Err(if cliopt::is_singlechar_option(opt) {
                    format!(
                        "cat: invalid option -- '{}'\n\
                         Try 'cat --help' for more information.",
                        opt.get(1..).unwrap()
                    )
                } else {
                    format!(
                        "cat: unrecognized option '{}'\n\
                         Try 'cat --help' for more information.",
                        opt
                    )
                })
            }
        }
        Ok(())
    }

    pub fn parse(&mut self, args: &[String]) -> Result<Vec<String>, String> {
        let mut files: Vec<String> = Vec::new();
        for arg in args {
            if cliopt::is_option(arg) {
                self.parse_option(arg)?;
            } else {
                files.push(arg.to_string());
            }
        }
        Ok(files)
    }

    fn help(&self, out_stream: &mut Write) {
        writeln!(out_stream, "help contents is here.").unwrap();
    }

    fn version(&self, out_stream: &mut Write) {
        writeln!(out_stream, "rust cat version 0.1.0").unwrap();
    }

    fn cat(
        &mut self,
        in_stream: &mut BufRead,
        out_stream: &mut Write,
        err_stream: &mut Write,
    ) -> Result<(), ()> {
        if self.option.number_noblank {
            // number nonempty output lines, overrides -n
            self.option.number = false;
        }

        let mut contents = String::new();
        loop {
            match in_stream.read_line(&mut contents) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    let newline_pat: &[_] = &['\n'];
                    contents = format!("{}", contents.trim_right_matches(newline_pat));
                    if self.option.squeeze_blank {
                        if contents.len() == 0 {
                            self.blank_count += 1;
                            if self.blank_count > 1 {
                                continue;
                            }
                        } else {
                            self.blank_count = 0;
                        }
                    }
                    if self.option.show_nonprinting {
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
                    if self.option.show_tabs {
                        contents = contents.replace("\t", "^I");
                    }
                    if (self.option.number_noblank && contents.len() > 0) || self.option.number {
                        contents = format!("{:>6}\t{}", self.number_count, contents);
                        self.number_count += 1;
                    }
                    if self.option.show_ends {
                        contents = format!("{}$", contents);
                    }
                    writeln!(out_stream, "{}", contents).unwrap();
                }
                Err(err) => {
                    writeln!(err_stream, "error occured! {:?}", err.kind()).unwrap();
                    return Err(());
                }
            }
            contents.clear();
        }
        Ok(())
    }

    pub fn run(
        &mut self,
        in_stream: &mut BufRead,
        out_stream: &mut Write,
        err_stream: &mut Write,
    ) -> Result<(), ()> {
        if self.option.display_version {
            self.version(out_stream);
            Err(())
        } else if self.option.display_help {
            self.help(out_stream);
            Err(())
        } else {
            self.cat(in_stream, out_stream, err_stream)
        }
    }
}
