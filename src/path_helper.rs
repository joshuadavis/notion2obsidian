use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use std::path::{Component, Path};

/// Converts the Option into a Result.
fn osstr_result(osstr: Option<&OsStr>) -> Result<&OsStr> {
    osstr.ok_or(anyhow!("OsStr was None!"))
}

/// Converts an OsStr to a &str (slice) yielding a Result (so you can use ?)
pub fn osstr_to_str(s: &OsStr) -> Result<&str> {
    s.to_str().ok_or(anyhow!("Invalid OsStr"))
}

/// Converts an OsStr into a String yielding a Result (so you can use ?)
pub fn osstr_to_string(osstr: Option<&OsStr>) -> Result<String> {
    Ok(String::from(osstr_to_str(osstr_result(osstr)?)?))
}

/// Converts a path to a string slice as a Result.
pub fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str().ok_or(anyhow!("Invalid Path!"))
}

/// Convert a path component into a String Result.
pub fn component_to_string(component: &std::path::Component) -> Result<String> {
    osstr_to_string(Some(component.as_os_str()))
}

/// Gets the file extension as a String, no default value.
pub fn get_extension(path: &Path) -> Result<String> {
    osstr_to_string(path.extension())
}

/// Gets just the file name part, without the extension.
pub fn get_file_stem(path: &Path) -> Result<String> {
    osstr_to_string(path.file_stem())
}

/// Returns Ok(true) if the file is a zip file.
pub fn is_zip_file(input_path: &Path) -> Result<bool> {
    Ok(get_extension(input_path)?.to_lowercase() == "zip")
}

// Returns Ok(true) if the file is a markdown file.
pub fn is_markdown_file(input_path: &Path) -> Result<bool> {
    Ok(get_extension(input_path)?.to_lowercase() == "md")
}

/// Gets the parent, as a Result so you can use ?.
pub fn get_parent(path: &Path) -> Result<&Path> {
    path.parent().ok_or(anyhow!("No parent!"))
}

/// Returns true if the links is "external" (e.g. a web link).
pub fn link_is_external(addr: &str) -> bool {
    addr.starts_with("http://")
        || addr.starts_with("https://")
        || addr.starts_with("about:")
        || addr.starts_with("mailto:")
}

/// Returns the component as a &str, or an error - but it always uses forward slashes, even on
/// Windows.
fn component_slash<'a>(c: &'a Component) -> Result<&'a str> {
    match c {
        Component::Normal(s) => osstr_to_str(s),
        Component::Prefix(p) => osstr_to_str(p.as_os_str()),
        Component::RootDir => Ok("/"),
        Component::CurDir => Ok("."),
        Component::ParentDir => Ok(".."),
    }
}

/// Converts a path to a string, but always uses forward slashes, even on Windows.
/// Obsidian only uses Unix style paths, so this is necessary.
pub fn path_slash(path: &Path) -> Result<String> {
    let mut strbuf = String::new();
    let last = path.components().count() - 1;
    for (i, c) in path.components().enumerate() {
        strbuf.push_str(component_slash(&c)?);
        if i < last {
            strbuf.push('/');
        }
    }
    Ok(strbuf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_slash() {
        if cfg!(windows) {
            let path = Path::new(r"foo\bar"); // Not sure if this works on Mac/Linux
            assert_eq!(path_slash(path).unwrap(), "foo/bar");
        } else {
            let path = Path::new("foo/bar");
            assert_eq!(path_slash(path).unwrap(), "foo/bar");
        }
    }

    #[test]
    fn test_link_is_external() {
        assert!(link_is_external("http://www.google.com"));
        assert!(link_is_external("https://www.google.com"));
        assert!(link_is_external("about:blank"));
        assert!(link_is_external("mailto:foo@bar.co"));
        assert!(!link_is_external("foo/bar.md"));
    }

    #[test]
    fn test_path_to_str() {
        let path = Path::new("/tmp/foo/bar");
        let s = path_to_str(path).unwrap();
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
        let strs = path
            .components()
            .map(|c| component_to_string(&c).unwrap())
            .collect::<Vec<String>>();
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
    fn test_get_file_stem() {
        let path = Path::new("foo/bar/test.tXt");
        assert_eq!(get_file_stem(&path).unwrap(), "test");
    }

    #[test]
    fn test_is_zip_file() {
        let path = Path::new("test.zip");
        assert!(is_zip_file(path).unwrap());
        let path = Path::new("test.txt");
        assert!(!is_zip_file(path).unwrap());
    }

    #[test]
    fn test_is_markdown_file() {
        let path = Path::new("test.md");
        assert!(is_markdown_file(path).unwrap());
        let path = Path::new("test.txt");
        assert!(!is_markdown_file(path).unwrap());
    }
}
