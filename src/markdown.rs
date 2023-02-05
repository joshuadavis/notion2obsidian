use crate::index::*;
use crate::links::{empty_is_none, fmt_md_link, fmt_wiki_link};
use crate::path_helper::{get_file_stem, get_parent, path_slash, path_to_str};
use crate::rex::*;
use crate::{file_helper, path_helper};
use anyhow::Result;
use file_helper::process_lines;
use lazy_static::lazy_static;
use log::{debug, info};
use path_helper::link_is_external;
use regex::Captures;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn get_link_value(found: &Captures, i: usize) -> Result<String> {
    let decoded = urlencoding::decode(get_capture_value(found, i)?)?;
    Ok(String::from(decoded.as_ref()))
}

fn get_new_link(
    is_image: bool,
    link_text: &str,
    link_addr: &Path,
    base_dir: &Path,
    index: &Index,
) -> Result<String> {
    let link_addr_string = path_to_str(link_addr)?;
    match index.find_by_path_or_relative_path(link_addr, base_dir) {
        Some(elem) => {
            let new_path = path_slash(elem.new_path.as_path())?;
            // If we found the address in the map, then use that with the 'internal link' syntax.
            if is_image {
                Ok(format!("![[{new_path}]]"))
            } else {
                Ok(fmt_wiki_link(&new_path, empty_is_none(link_text)))
            }
        }
        None => {
            if link_is_external(link_addr_string) {
                // If the link is external, then use the markdown syntax.
                // If there is no link text, then just use the link address.
                Ok(fmt_md_link(link_addr_string, empty_is_none(link_text)))
            } else {
                // Otherwise,this is some kind of non-external link that isn't in the index.
                info!(
                    "Link not found: {}, assuming external link",
                    path_to_str(link_addr)?
                );
                Ok(fmt_md_link(
                    path_to_str(link_addr)?,
                    empty_is_none(link_text),
                ))
            }
        }
    }
}

/// State - updated while processing lines in the markdown file.
struct State {
    headers_processed: usize,
    links_processed: usize,
    tags: Vec<String>,
}

fn process_links(state: &mut State, line: &str, paths: &Paths, index: &Index) -> Result<String> {
    lazy_static! {
        static ref RE_LINK: Regex = Regex::new(r"!?\[(.*)\]\((.*)\)").unwrap();
    }

    let mut new_line = String::from(line);
    for (i, found) in RE_LINK.captures_iter(&line).enumerate() {
        if found.len() == 3 {
            // The three parts are: 0 = the whole match, 1 = the link text, 2 = the link address.
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
        static ref RE_HEADER: Regex = Regex::new(r"^(#+)\s+(.*)").unwrap();
    }

    if let Some(c) = RE_HEADER.captures(line) {
        let level = get_capture_value(&c, 1)?.len();
        let text = get_capture_value(&c, 2)?;
        state.headers_processed += 1;
        if state.headers_processed == 1 && level == 1 && text == get_file_stem(&paths.new_path)? {
            debug!("Deleting redundant header: {}, {}", level, text);
            return Ok(String::from(""));
        }
    }
    Ok(String::from(line))
}

fn prepare_tag(tag: &String) -> String {
    let mut tag = tag.clone(); // Clone, so we can use mutable methods.
    tag.retain(|c| c.is_alphabetic() || c.is_numeric() || c == '_');
    tag.insert(0, '#');
    tag
}

fn create_tag_string(tags: &Vec<String>) -> String {
    tags.iter()
        .map(prepare_tag)
        .collect::<Vec<String>>() // Collect into a Vec<String>, so we can join.
        .join(" ")
}

fn process_tags(state: &mut State, line: &str) -> Result<()> {
    lazy_static! {
        static ref RE_TAGS: Regex = Regex::new("^[Tt]ags: (.+)").unwrap();
    }
    if let Some(c) = RE_TAGS.captures(line) {
        get_capture_value(&c, 1)?.split(",").for_each(|tag| {
            state.tags.push(String::from(tag.trim()));
        });
    }
    Ok(())
}

fn process_line(state: &mut State, line: &str, paths: &Paths, index: &Index) -> Result<String> {
    process_tags(state, line)?;
    let line = process_header(state, line, paths)?;
    process_links(state, &line, paths, index)
}

pub fn process_markdown(paths: &Paths, index: &Index) -> Result<()> {
    let mut state = State {
        headers_processed: 0,
        links_processed: 0,
        tags: vec![],
    };

    let output_path = paths.output_path();
    process_lines(paths.input_path().as_path(), &output_path, |line| {
        process_line(&mut state, line, paths, index)
    })?;

    // If there are tags to write, append them to the output file.
    if !state.tags.is_empty() {
        let file = OpenOptions::new().append(true).open(&output_path)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer)?;
        writeln!(writer, "---")?;
        writeln!(writer, "Tags: {}", create_tag_string(&state.tags))?;
        writer.flush()?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_dir_all, remove_file};

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
        let (dir, output_dir, index) =
            prepare_input("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4");
        let paths = get_paths(
            dir,
            &output_dir,
            &index,
            "Kotlin ff533adf543444398ee04488cfa17db1.md",
        );
        process_markdown(&paths, &index).unwrap();
        cleanup(&output_dir);
    }

    #[test]
    fn test_get_new_link() {
        let (dir, output_dir, index) =
            prepare_input("test-data/Documentation c5b82e1ba6e94f87bb3f537f639378b4");
        let paths = get_paths(
            dir,
            &output_dir,
            &index,
            "Kotlin ff533adf543444398ee04488cfa17db1.md",
        );
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
        let (dir, output_dir, index) =
            prepare_input("test-data/My Links 4d87e5fbcac64818adbd9511585bd720");
        let elem = index.find_by_path(Path::new("DIY Guitar Effects Pedal and Amplifier Kits – Buil b8cb99f4968a402bae2a08b90d9c0123.md")).unwrap();
        println!("{elem:?}");
        let new_path = Path::new("target/output/link-test.md");
        if new_path.exists() {
            remove_file(new_path).unwrap();
        }
        let base_dir = dir;
        let paths = Paths::from_elem(elem, base_dir, &output_dir);
        process_markdown(&paths, &index).unwrap();
        // TODO: Verify the output.
        cleanup(&output_dir);
    }
}
