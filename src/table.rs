use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufWriter, Write};
use anyhow::Result;

use crate::file_helper::{open_output_file};
use crate::index::Index;
use crate::path_helper::path_to_string;

pub fn compute_link_addr(path: &Path, name: &str, index: &Index) -> Option<PathBuf> {
    let mut path = path.to_path_buf();
    path.set_extension("");
    path.push(name);
    path.set_extension("md");
    let maybe_elem = index.find_by_output_path(&path);
    if let Some(elem) = maybe_elem {
        Some(elem.output_path.clone())
    } else {
        None
    }
}

fn write_headers(reader : &mut csv::Reader<File>, writer: &mut BufWriter<File>) -> Result<bool> {
    let headers = reader.headers()?;
    let mut header_lengths : Vec<usize> = Vec::new();
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

pub fn convert_csv_to_markdown(new_path: &Path,input: &Path, output: &Path, index: &Index) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut writer = open_output_file(output)?;
    let mut link_first_column = false;

    // First, write the headers.
    if reader.has_headers() {
        link_first_column = write_headers(&mut reader, &mut writer)?;
    }

    for row in reader.records() {
        let row = row?;
        for (column, field) in row.iter().enumerate() {
            if link_first_column &&  column == 0 {
                if let Some(link_addr) = compute_link_addr(new_path, field, index) {
                    write!(writer, "|[[{}|{}]]", path_to_string(&link_addr)?, field)?;
                } else {
                    write!(writer, "|{}", field)?;
                }
            } else {
                write!(writer, "|{}", field)?;
            }
        }
        writeln!(writer, "|")?;
    }

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