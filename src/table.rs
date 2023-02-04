use anyhow::Result;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use log::info;

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

fn write_headers<T: Write, U: Read>(reader: &mut csv::Reader<U>, writer: &mut BufWriter<T>) -> Result<Vec<String>> {
    let headers = reader.headers()?;
    let mut header_lengths: Vec<usize> = Vec::new();
    let mut header_strings: Vec<String> = Vec::new();

    for (idx, header) in headers.iter().enumerate() {
        write!(writer, "|{}", header)?;
        header_lengths.push(header.len());
        header_strings.push(String::from(header))
    }

    writeln!(writer, "|")?;
    for header_length in header_lengths {
        write!(writer, "|{}", "-".repeat(header_length))?;
    }
    writeln!(writer, "|")?;
    Ok(link_first_column)
}

fn write_link<T: Write>(writer: &mut BufWriter<T>, link_addr: &Path) -> Result<()> {
    // We can't use the full "wikilink" Obsidian format here, as the vertical bar will
    // mess up the table.  Just use a wikilink with the new path.
    let addr = path_to_str(&link_addr)?;
    let f = format!("[[{}]]", addr);
    write_field(writer, &f)
}

fn write_field<T: Write>(writer: &mut BufWriter<T>, field: &str) -> Result<()> {
    let field = if field.is_empty() { " " } else { field };
    write!(writer, "| {}", field)?;
    Ok(())
}

fn write_files<T: Write>(writer: &mut BufWriter<T>, field: &str,index: &Index) -> Result<()> {
    // Parse the field - it should be a comma-separated list.
    write!(writer, "| ")?;
    for f in field.split(",").map(|x| { x.trim() }) {
        // TODO: Look up the file in the index and write it as a link if we find it.

        // Otherwise, just print it out.
        write!(writer, "{}", f)?;
    }
    Ok(())
}

pub fn convert_csv_to_markdown(paths: &index::Paths, index: &Index) -> Result<()> {
    let input = paths.input_path();
    let output = paths.output_path();
    let new_path = &paths.new_path;

    let mut reader = csv::Reader::from_path(&input)?;
    let mut writer = open_output_file(&output)?;

    // First, write the headers.
    let headers = write_headers(&mut reader, &mut writer)?;

    for row in reader.records() {
        let row = row?;
        // Iterate through the cells of the row, with the header of each cell.
        for cell_with_header in row.iter().enumerate().zip( // Join the 'column/cell' pair with...
            headers.iter().map(|h| { h.as_str() })  // Turn the headers into slices.
        ) {
            let ((column, field), header) = cell_with_header;
            if column == 0 && header == "Name" {
                if let Some(link_addr) = compute_link_addr(new_path, field, index) {
                    write_link(&mut writer, &link_addr)?;
                } else {
                    info!("Link not found: {}", field);
                    write_field(&mut writer, field)?;
                }
            } else if header == "Files" {
                write_files(&mut writer, field, index)?;
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
