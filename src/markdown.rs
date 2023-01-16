use anyhow::{Result};
use regex::Regex;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use log::{debug, info};
use regex::Captures;
use file_helper::process_lines;
use crate::file_helper;
use crate::path_helper::{get_file_stem, get_parent, path_to_string};
use crate::index::*;
use crate::rex::*;

fn get_link_value(found: &Captures, i: usize) -> Result<String> {
    let decoded = urlencoding::decode(get_capture_value(found, i)?)?;
    Ok(String::from(decoded.as_ref()))
}

fn link_is_external(addr: &str) -> bool {
    addr.starts_with("http://") || addr.starts_with("https://") || addr.starts_with("about:") || addr.starts_with("mailto:")
}

fn get_new_link(is_image: bool, link_text: &str, link_addr: &Path, base_dir: &Path, index: &Index) -> Result<String> {
    let link_addr_string = path_to_string(link_addr)?;
    match index.find_by_path_or_relative_path(link_addr, base_dir) {
        Some(elem) => {
            let new_path = path_to_string(elem.new_path.as_path())?;
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

struct State {
    headers_processed: usize,
    links_processed: usize,
}

fn process_links(state: &mut State, line: &str, paths: &Paths, index: &Index) -> Result<String> {
    lazy_static! {
        static ref RE_LINK: Regex = Regex::new(
            r"!?\[(.*)\]\((.*)\)")
        .unwrap();
    }

    let mut new_line = String::from(line);
    for (i, found) in RE_LINK.captures_iter(&line).enumerate() {
        if found.len() == 3 { // The three parts are: 0 = the whole match, 1 = the link text, 2 = the link address.
            let original = get_capture_value(&found, 0)?;
            let is_image = original.starts_with("!");
            let link_text = get_capture_value(&found, 1)?;
            let link_addr = PathBuf::from(get_link_value(&found, 2)?);
            let base_dir = get_parent(&paths.old_path)?;
            let new_link = get_new_link(is_image, link_text, &link_addr, base_dir, index)?;
            new_line = line.replace(original, new_link.as_str());
            state.links_processed += 1;
            debug!("[{}] {} -> {}", i, original, new_link);
        }
    }
    Ok(new_line)
}

fn process_header(state: &mut State, line: &str, paths: &Paths) -> Result<String> {
    lazy_static! {
    static ref RE_HEADER: Regex = Regex::new(
        r"^(#+)\s+(.*)")
        .unwrap();
    }

    if let Some(c) = RE_HEADER.captures(line) {
        let level = get_capture_value(&c, 1)?.len();
        let text = get_capture_value(&c, 2)?;
        state.headers_processed += 1;
        if state.headers_processed == 1 && level == 1 && text == get_file_stem(&paths.new_path)? {
            debug!("Deleting redundant header: {}, {}", level, text);
            return Ok(String::from(""))
        }
    }
    Ok(String::from(line))
}

fn process_line(state: &mut State, line: &str, paths: &Paths, index: &Index) -> Result<String> {
    let line = process_header(state, line, paths)?;
    process_links(state, &line, paths, index)
}

pub fn process_markdown(paths: &Paths, index: &Index) -> Result<()> {
    let mut state = State {
        headers_processed: 0,
        links_processed: 0,
    };
    process_lines(paths.input_path().as_path(),
                  paths.output_path().as_path(),
                  |line| process_line(&mut state, line, paths, index))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_dir_all, remove_file};
    use super::*;

    #[test]
    fn test_create_path_map() {
        // A simple folder with three markdown files in it.
        let dir = Path::new("test-data/folder1");
        let path_map = Index::from_dir(dir).unwrap();
        assert_eq!(path_map.len(), 3);
    }

    fn prepare_input(input_dir: &str) -> (&Path, PathBuf, Index) {
        let dir = Path::new(input_dir);
        let output_dir = tempfile::tempdir().unwrap();
        let index = Index::from_dir(dir).unwrap();
        debug!("Output dir: {}", output_dir.path().display());
        (dir, output_dir.into_path(), index)
    }

    fn cleanup(output_dir: &PathBuf) {
        debug!("Cleaning up: {}", output_dir.display());
        remove_dir_all(&output_dir).unwrap();
    }

    fn get_paths(dir: &Path, output_dir: &Path, index: &Index, document: &str) -> Paths {
        let old_path = Path::new(document);
        let elem = index.find_by_path(old_path).unwrap();
        Paths::from_elem(elem, dir, output_dir)
    }

    #[test]
    fn test_process_markdown() {
        let (dir, output_dir, index) = prepare_input("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4");
        let paths = get_paths(dir, &output_dir, &index, "Kotlin ff533adf543444398ee04488cfa17db1.md");
        process_markdown(&paths, &index).unwrap();
        cleanup(&output_dir);
    }

    #[test]
    fn test_get_new_link() {
        let (dir, output_dir, index) = prepare_input("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4");
        let paths = get_paths(dir, &output_dir, &index, "Kotlin ff533adf543444398ee04488cfa17db1.md");
        let link_text = "Kotlin";
        let link_addr = Path::new("Kotlin ff533adf543444398ee04488cfa17db1.md");
        let base_dir = get_parent(&paths.old_path).unwrap();
        let new_link = get_new_link(false, link_text, link_addr, base_dir, &index).unwrap();
        assert_eq!(new_link, "[[Kotlin.md|Kotlin]]");

        let link_text = "YouTube";
        let link_addr = Path::new("https://www.youtube.com");
        let new_link = get_new_link(false, link_text, link_addr, base_dir, &index).unwrap();
        assert_eq!(new_link, "[YouTube](https://www.youtube.com)");

        let link_text = "";
        let link_addr = Path::new("https://www.youtube.com");
        let new_link = get_new_link(false, link_text, link_addr, base_dir, &index).unwrap();
        assert_eq!(new_link, "https://www.youtube.com");
        cleanup(&output_dir);
    }

    #[test]
    fn test_image_link() {
        let (dir, output_dir, index) = prepare_input("test-data/My Links 4d87e5fbcac64818adbd9511585bd720");
        let elem = index.find_by_path(Path::new("DIY Guitar Effects Pedal and Amplifier Kits – Buil b8cb99f4968a402bae2a08b90d9c0123.md")).unwrap();
        println!("{:?}", elem);
        let new_path = Path::new("target/output/link-test.md");
        if new_path.exists() {
            remove_file(new_path).unwrap();
        }
        let base_dir = dir;
        let paths = Paths::from_elem(elem, base_dir, &output_dir);
        process_markdown(&paths, &index).unwrap();
        cleanup(&output_dir);
    }
}