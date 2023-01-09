
use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use lazy_static::lazy_static;
use regex::Regex;
use crate::path_helper;

pub fn replace_hex(s: &OsStr) -> Result<String> {
    lazy_static! {
        static ref RE_FILENAME: Regex = Regex::new(
            r"(.*)( +[0-9a-f]+)(\..*)")
        .unwrap();
        static ref RE_DIR: Regex = Regex::new(
            r"(.*)( +[0-9a-f]+)")
        .unwrap();
    }
    let text = path_helper::osstr_to_str(s)?;
    if text.contains(".") {
        Ok(RE_FILENAME.replace(path_helper::osstr_to_str(s)?, "$1$3").to_string())
    } else {
        Ok(RE_DIR.replace(path_helper::osstr_to_str(s)?, "$1").to_string())
    }
}

pub fn get_capture_value<'a>(capture: &'a regex::Captures, i:usize) -> Result<&'a str> {
    Ok(capture.get(i).ok_or(anyhow!("Invalid  capture {}!", i))?.as_str())
}
