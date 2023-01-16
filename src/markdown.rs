use anyhow::{Result};
use regex::Regex;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use log::{debug, info};
use regex::Captures;
use file_helper::process_lines;
use crate::file_helper;
use crate::path_helper::path_to_string;
use crate::index::*;
use crate::rex::*;

fn get_link_value(found: &Captures, i: usize) -> Result<String> {
    let decoded = urlencoding::decode(get_capture_value(found, i)?)?;
    Ok(String::from(decoded.as_ref()))
}

fn link_is_external(addr: &str) -> bool {
    addr.starts_with("http://") || addr.starts_with("https://")
}

fn get_new_link(is_image: bool, link_text: &str, link_addr: &Path, index: &Index) -> Result<String> {
    let link_addr_string = path_to_string(link_addr)?;
    match index.find_by_path(link_addr) {
        Some(elem) => {
            let new_path = path_to_string(elem.output_path.as_path())?;
            // If we found the address in the map, then use that with the 'internal link' syntax.
            if is_image {
                Ok(format!("![[{}]]", new_path))
            } else {
                Ok(format!("[[{}|{}]]", new_path, link_text))
            }
        }
        None => {
            if link_is_external(link_addr_string.as_str()) {
                // If the link is external, then use the 'external link' syntax.
                // If there is no link text, then just use the link address.
                if link_text.is_empty() {
                    Ok(link_addr_string)
                } else {
                    Ok(format!("[{}]({})", link_text, link_addr_string))
                }
            } else {
                // Otherwise,this is some kind of non-external link that isn't in the index.
                info!("Link not found: {}, assuming external link", path_to_string(link_addr)?);
                Ok(format!("[{}]({})", link_text, path_to_string(link_addr)?))
            }
        }
    }
}

fn process_line(line: &str, index: &Index) -> Result<String> {
    lazy_static! {
        static ref RE_LINK: Regex = Regex::new(
            r"!?\[(.*)\]\((.*)\)")
        .unwrap();
    }
    // Look for links.
    let mut new_line = String::from(line);
    for (i, found) in RE_LINK.captures_iter(&line).enumerate() {
        if found.len() == 3 {
            let original = get_capture_value(&found, 0)?;
            let is_image = original.starts_with("!");
            let link_text = get_capture_value(&found, 1)?;
            let link_addr = PathBuf::from(get_link_value(&found, 2)?);
            let new_link = get_new_link(is_image, link_text, &link_addr, index)?;
            new_line = line.replace(original, new_link.as_str());
            debug!("[{}] {} -> {}", i, original, new_link);
        }
    }
    Ok(new_line)
}

pub fn process_markdown(old_path: &Path, new_path: &Path,
                        path_map: &Index) -> Result<()> {
    process_lines(old_path, new_path,
                  |line| process_line(line, path_map))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::file_helper::remove_file_if_exists;
    use super::*;

    #[test]
    fn test_create_path_map() {
        // A simple folder with three markdown files in it.
        let dir = Path::new("test-data/folder1");
        let path_map = Index::from_dir(dir).unwrap();
        assert_eq!(path_map.len(),  3);
    }

    #[test]
    fn test_process_markdown() {
        let old_path = Path::new("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4/Kotlin ff533adf543444398ee04488cfa17db1.md");
        let new_path= Path::new("target/test-data/Documentation/Kotlin.md");
        remove_file_if_exists(new_path).unwrap();
        let dir = Path::new("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4");
        let index = Index::from_dir(dir).unwrap();
        process_markdown(&old_path, &new_path, &index).unwrap();
    }

    #[test]
    fn test_get_new_link() {
        let dir = Path::new("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4");
        let path_map = Index::from_dir(dir).unwrap();
        let link_text = "Kotlin";
        let link_addr = Path::new("Kotlin ff533adf543444398ee04488cfa17db1.md");
        let new_link = get_new_link(false, link_text, link_addr, &path_map, ).unwrap();
        assert_eq!(new_link, "[[Kotlin.md|Kotlin]]");

        let link_text = "YouTube";
        let link_addr = Path::new("https://www.youtube.com");
        let new_link = get_new_link(false, link_text, link_addr, &path_map, ).unwrap();
        assert_eq!(new_link, "[YouTube](https://www.youtube.com)");

        let link_text = "";
        let link_addr = Path::new("https://www.youtube.com");
        let new_link = get_new_link(false, link_text, link_addr, &path_map, ).unwrap();
        assert_eq!(new_link, "https://www.youtube.com");
    }

    #[test]
    fn test_image_link() {
        let dir = Path::new("test-data/My Links 4d87e5fbcac64818adbd9511585bd720");
        let index = Index::from_dir(dir).unwrap();
        let elem = index.find_by_path(Path::new("DIY Guitar Effects Pedal and Amplifier Kits – Buil b8cb99f4968a402bae2a08b90d9c0123.md")).unwrap();
        println!("{:?}", elem);
        let new_path = Path::new("target/output/link-test.md");
        remove_file_if_exists(new_path).unwrap();
        let input_path = dir.join(elem.path.as_path());
        process_markdown(&input_path, new_path, &index).unwrap();
    }
}