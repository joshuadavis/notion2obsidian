use anyhow::{anyhow, bail, Result};
use std::ffi::OsStr;
use std::path::{Path};

/// Converts the Option into a Result.
fn osstr_result(osstr: Option<&OsStr>) -> Result<&OsStr> {
    if osstr.is_none() {
        bail!("No OsStr?")
    } else {
        Ok(osstr.unwrap())
    }
}

/// Converts an OsStr to a &str (slice) yielding a Result (so you can use ?)
pub fn osstr_to_str(s: &OsStr) -> Result<&str> {
    s.to_str().ok_or(anyhow!("Invalid OsStr"))
}

/// Converts an OsStr into a String yielding a Result (so you can use ?)
pub fn osstr_to_string(osstr: Option<&OsStr>) -> Result<String> {
    Ok(String::from(osstr_to_str(osstr_result(osstr)?)?))
}

/// Convert a Path to a String as a Result (so you can use ?)
pub fn path_to_string(path: &Path) -> Result<String> {
    Ok(String::from(path.to_str().ok_or(anyhow!("Invalid Path!"))?))
}

/// Convert a path component into a String Result.
pub fn component_to_string(component: &std::path::Component) -> Result<String> {
    osstr_to_string(Some(component.as_os_str()))
}

/// Gets the file extension as a String, no default value.
pub fn get_extension(path: &Path) -> Result<String> {
    osstr_to_string(path.extension())
}

pub fn is_zip_file(input_path: &Path) -> Result<bool> {
    Ok(input_path.is_file() && get_extension(input_path)?.to_lowercase() == "zip")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_string() {
        let path = Path::new("/tmp/foo/bar");
        let s = path_to_string(path).unwrap();
        assert_eq!(s, "/tmp/foo/bar");
    }

    #[test]
    fn test_osstr_to_string() {
        let osstr = OsStr::new("foo");
        let s = osstr_to_string(Some(osstr)).unwrap();
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_component_to_str() {
        let path = Path::new("/tmp/foo/bar");
        let strs = path.components().map(|c| {
            component_to_string(&c).unwrap()
            }).collect::<Vec<String>>();
        assert_eq!(strs.len(), 4);
        assert_eq!(strs[1], "tmp");
        assert_eq!(strs[2], "foo");
        assert_eq!(strs[3], "bar");
    }

    #[test]
    fn test_get_extension() {
        let path = Path::new("test.tXt");
        assert_eq!(get_extension(&path).unwrap().to_lowercase(), "txt");
    }

    #[test]
    fn test_is_zip_file() {
        let path = Path::new("test.zip");
        assert!(is_zip_file(&path).unwrap());
    }

}
