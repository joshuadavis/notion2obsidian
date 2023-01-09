use std::path::Path;
use std::io::Write;

use crate::file_helper::{open_output_file};

pub fn convert_csv_to_markdown(old_path: &Path, new_path: &Path) -> anyhow::Result<()> {

    let mut reader = csv::Reader::from_path(old_path)?;
    let mut writer = open_output_file(new_path)?;

    // First, write the headers.
    if reader.has_headers() {
        let headers = reader.headers()?;
        let mut header_lengths : Vec<usize> = Vec::new();
        for header in headers {
            write!(writer, "|{}", header)?;
            header_lengths.push(header.len());
        }
        writeln!(writer, "|")?;
        for header_length in header_lengths {
            write!(writer, "|{}", "-".repeat(header_length))?;
        }
        writeln!(writer, "|")?;
    }

    for row in reader.records() {
        let row = row?;
        for field in row.iter() {
            write!(writer, "|{}", field)?;
        }
        writeln!(writer, "|")?;
    }

    writeln!(writer, "----")?;
    writeln!(writer, "#notion2obsidian #csvimport")?;
    writer.flush()?;
    Ok(())
}
