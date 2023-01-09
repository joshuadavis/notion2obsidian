use std::path::Path;
use std::io::Write;
use log::info;

use crate::file_helper::{open_output_file};

pub fn convert_csv_to_markdown(old_path: &Path, new_path: &Path) -> anyhow::Result<()> {

    let mut reader = csv::Reader::from_path(old_path)?;
    let mut writer = open_output_file(new_path)?;
    let mut link_first_column = false;

    // First, write the headers.
    if reader.has_headers() {
        let headers = reader.headers()?;
        let mut header_lengths : Vec<usize> = Vec::new();
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
    }

    for row in reader.records() {
        let row = row?;
        for (column, field) in row.iter().enumerate() {
            if link_first_column &&  column == 0 {
                info!("Trying to link {}", field);
                write!(writer, "|{}", field)?;
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