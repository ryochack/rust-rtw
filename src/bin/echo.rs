use std::env;

fn unescape(s: &[String]) {
    for (i, v) in s.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        let mut backslashed = false;
        for c in v.chars() {
            if c == '\\' && !backslashed {
                backslashed = true;
                continue;
            }
            if backslashed {
                match c {
                    'n' => print!("\n"),
                    'r' => print!("\r"),
                    't' => print!("\t"),
                    _ => print!("\\{}", c),
                }
                backslashed = false;
            } else {
                print!("{}", c);
            }
        }
        if backslashed {
            print!("\\");
        }
    }
}

fn raw(s: &[String]) {
    for (i, v) in s.iter().enumerate() {
        print!("{}{}",
               if i == 0 {
                   ""
               } else {
                   " "
               }, v);
    }
}

fn main() {
    let mut output_trailing_newline = true;
    let mut enable_interp_backslash_escapes = true;

    let args: Vec<String> = env::args().skip(1).collect();
    let mut s = args.as_slice();

    for (i, v) in s.iter().enumerate() {
        match v.as_str() {
            "-e" => enable_interp_backslash_escapes = true,
            "-E" => enable_interp_backslash_escapes = false,
            "-n" => output_trailing_newline = false,
            _ => {
                s = &s[i..];
                break;
            }
        }
    }

    if enable_interp_backslash_escapes {
        unescape(&s);
    } else {
        raw(&s);
    }
    if output_trailing_newline {
        println!();
    }
}
