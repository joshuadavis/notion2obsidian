use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

pub fn replace_hex(s: &str) -> Result<String> {
    lazy_static! {
        static ref RE_FILENAME: Regex = Regex::new(
            r"(.*)( +[0-9a-fA-F]+)(\..*)").unwrap();
        static ref RE_DIR: Regex = Regex::new(
            r"(.*)( +[0-9a-fA-F]+)").unwrap();
    }
    // Replace hex strings in filenames and directory names.
    let rv = if s.contains(".") {
        RE_FILENAME.replace(s, "$1$3").to_string()
    } else {
        RE_DIR.replace(s, "$1").to_string()
    };
    // Trim off trailing spaces, and return.  Windows doesn't like trailing spaces.
    Ok(rv.trim().to_string())
}

pub fn get_capture_value<'a>(capture: &'a regex::Captures, i: usize) -> Result<&'a str> {
    Ok(capture.get(i).ok_or(anyhow!("Invalid  capture {}!", i))?.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_hex() {
        assert_eq!(replace_hex("foo 1234EEABC.txt").unwrap(), "foo.txt");
        assert_eq!(replace_hex("foo 1234ffd3eeaA12").unwrap(), "foo");
        assert_eq!(replace_hex("foo 1234FFD12  ").unwrap(), "foo");
    }
}