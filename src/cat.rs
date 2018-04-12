#![allow(dead_code)]
use super::cliopt;
use std::io::prelude::*;

#[derive(Clone, Default, PartialEq, Debug)]
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
            option: CmdOption::default(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, SeekFrom};
    use std::process;

    #[test]
    fn test_build() {
        let c = CatBuilder::new().build();
        assert_eq!(CmdOption::default(), c.option);

        let c = CatBuilder::new().with_show_all().build();
        assert_eq!(
            CmdOption {
                show_ends: true,
                show_tabs: true,
                show_nonprinting: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_number_nonblank().build();
        assert_eq!(
            CmdOption {
                number_noblank: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_e().build();
        assert_eq!(
            CmdOption {
                show_ends: true,
                show_nonprinting: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_show_ends().build();
        assert_eq!(
            CmdOption {
                show_ends: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_number().build();
        assert_eq!(
            CmdOption {
                number: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_squeeze_blank().build();
        assert_eq!(
            CmdOption {
                squeeze_blank: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_t().build();
        assert_eq!(
            CmdOption {
                show_tabs: true,
                show_nonprinting: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_show_tabs().build();
        assert_eq!(
            CmdOption {
                show_tabs: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_show_nonprinting().build();
        assert_eq!(
            CmdOption {
                show_nonprinting: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_display_help().build();
        assert_eq!(
            CmdOption {
                display_help: true,
                ..Default::default()
            },
            c.option
        );

        let c = CatBuilder::new().with_display_version().build();
        assert_eq!(
            CmdOption {
                display_version: true,
                ..Default::default()
            },
            c.option
        );
    }

    #[test]
    fn test_parse() {
        {
            // "-A" | "--show-all"
            let expects = CmdOption {
                show_ends: true,
                show_tabs: true,
                show_nonprinting: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-A".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--show-all".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-b" | "--number-nonblank"
            let expects = CmdOption {
                number_noblank: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-b".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--number-nonblank".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-e"
            let expects = CmdOption {
                show_ends: true,
                show_nonprinting: true,
                ..Default::default()
            };
            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-e".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-E" | "--show-ends"
            let expects = CmdOption {
                show_ends: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-E".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--show-ends".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-n" | "--number"
            let expects = CmdOption {
                number: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-n".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--number".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-s" | "--squeeze-blank"
            let expects = CmdOption {
                squeeze_blank: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-s".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--squeeze-blank".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-t"
            let expects = CmdOption {
                show_tabs: true,
                show_nonprinting: true,
                ..Default::default()
            };
            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-t".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-T" | "--show-tabs"
            let expects = CmdOption {
                show_tabs: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-T".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--show-tabs".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-u"
            let expects = CmdOption::default();
            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-u".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "-v" | "--show-nonprinting"
            let expects = CmdOption {
                show_nonprinting: true,
                ..Default::default()
            };

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["-v".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));

            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--show-nonprinting".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "--help"
            let expects = CmdOption {
                display_help: true,
                ..Default::default()
            };
            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--help".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }

        {
            // "--version"
            let expects = CmdOption {
                display_version: true,
                ..Default::default()
            };
            let mut c = CatBuilder::new().build();
            let files = c.parse(&["--version".to_string()]);
            assert_eq!(expects, c.option);
            assert_eq!(files, Ok(Vec::new()));
        }
    }

    #[test]
    fn test_cat() {
        const TEST_DATA_PATH: &str = "ci-tests/test-data/cat_test.txt";
        let mut file = File::open(&TEST_DATA_PATH).expect(&format!(
            "cat: {}: No such file or directory",
            TEST_DATA_PATH
        ));
        let options = ["-A", "-b", "-e", "-E", "-n", "-s", "-t", "-T", "-u", "-v"];

        for o in options.iter() {
            let mut c = CatBuilder::new().build();
            let _ = c.parse(&[o.to_string()]);
            let mut outstream: Vec<u8> = Vec::new();
            let mut errstream: Vec<u8> = Vec::new();

            {
                let mut freader = BufReader::new(&file);
                c.run(&mut freader, &mut outstream, &mut errstream)
                    .expect("Failed to execute command");
                let expects = process::Command::new("cat")
                    .arg(o)
                    .arg(TEST_DATA_PATH)
                    .output()
                    .expect("Failed to execute command");
                assert_eq!(outstream, expects.stdout, "test with '{}' option", o);
            }

            file.seek(SeekFrom::Start(0)).expect("Failed to seek file");
        }
    }
}
