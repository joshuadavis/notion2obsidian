use anyhow::Result;
use std::io::{BufWriter, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use crate::file_helper::open_output_file;
use crate::index;
use crate::index::Index;
use crate::path_helper::{path_to_str};

pub fn compute_link_addr(path: &Path, name: &str, index: &Index) -> Option<PathBuf> {
    let mut path = path.to_path_buf();
    path.set_extension("");
    path.push(name);
    path.set_extension("md");
    let maybe_elem = index.find_by_output_path(&path);
    if let Some(elem) = maybe_elem {
        Some(elem.new_path.clone())
    } else {
        None
    }
}

fn write_headers<T: Write, U: Read>(reader: &mut csv::Reader<U>, writer: &mut BufWriter<T>) -> Result<bool> {
    let headers = reader.headers()?;
    let mut header_lengths: Vec<usize> = Vec::new();
    let mut link_first_column = false;
    for (idx, header) in headers.iter().enumerate() {
        write!(writer, "|{}", header)?;
        header_lengths.push(header.len());
        // If the first column is "Name", try to transform the column values into links.
        if idx == 0 && header == "Name" {
            link_first_column = true;
        }
    }
    writeln!(writer, "|")?;
    for header_length in header_lengths {
        write!(writer, "|{}", "-".repeat(header_length))?;
    }
    writeln!(writer, "|")?;
    Ok(link_first_column)
}

fn write_link<T: Write>(writer: &mut BufWriter<T>, field: &str, link_addr: &Path) -> Result<()> {
    // We can't use the "wikilink" Obsidian format here, as the vertical bar will
    // mess up the table.
    let addr = urlencoding::encode(path_to_str(&link_addr)? );
    let f = format!("[{}]({})", field, addr.deref());
    write_field(writer, &f)
}

fn write_field<T: Write>(writer: &mut BufWriter<T>, field: &str) -> Result<()> {
    let field = if field.is_empty() { " " } else { field };
    write!(writer, "|{}", field)?;
    Ok(())
}

pub fn convert_csv_to_markdown(paths: &index::Paths, index: &Index) -> Result<()> {
    let input = paths.input_path();
    let output = paths.output_path();
    let new_path = &paths.new_path;

    let mut reader = csv::Reader::from_path(&input)?;
    let mut writer = open_output_file(&output)?;
    let mut link_first_column = false;  // True if the first column should be interpreted as a link.

    // First, write the headers.
    if reader.has_headers() {
        link_first_column = write_headers(&mut reader, &mut writer)?;
    }

    for row in reader.records() {
        let row = row?;
        for (column, field) in row.iter().enumerate() {
            if link_first_column && column == 0 {
                if let Some(link_addr) = compute_link_addr(new_path, field, index) {
                    write_link(&mut writer, field, &link_addr)?;
                } else {
                    write_field(&mut writer, field)?;
                }
            } else {
                write_field(&mut writer, field)?;
            }
        }
        writeln!(writer, "|")?;
    }

    writeln!(writer, "")?;
    writeln!(writer, "")?;
    writeln!(writer, "----")?;
    writeln!(writer, "#notion2obsidian #csvimport")?;
    writer.flush()?;
    Ok(())
}


#[cfg(test)]
mod test {
    #[test]
    fn test_convert_csv_to_markdown() {
        assert!(true);
    }
}
