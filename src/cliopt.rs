#![allow(dead_code)]

pub fn is_option(arg: &str) -> bool {
    arg.len() > 1 && arg.chars().nth(0) == Some('-')
}

pub fn is_singlechar_option(arg: &str) -> bool {
    arg.len() > 1 && arg.chars().nth(0) == Some('-') && arg.chars().nth(1) != Some('-')
}

pub fn is_multichar_option(arg: &str) -> bool {
    arg.starts_with("--")
}
