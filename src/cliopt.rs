#![allow(dead_code)]
use std::ffi::OsStr;

pub fn is_option(arg: &OsStr) -> bool {
    arg.len() > 1 && arg.to_str().unwrap().chars().nth(0) == Some('-')
}

pub fn is_singlechar_option(arg: &OsStr) -> bool {
    arg.len() > 1 && arg.to_str().unwrap().chars().nth(0) == Some('-')
        && arg.to_str().unwrap().chars().nth(1) != Some('-')
}

pub fn is_multichar_option(arg: &OsStr) -> bool {
    arg.to_str().unwrap().starts_with("--")
}
