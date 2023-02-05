use anyhow::Result;
use log::{debug, info};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use crate::file_helper::open_output_file;
use crate::index;
use crate::index::Index;
use crate::links::{empty_is_none, fmt_wiki_link};
use crate::path_helper::{
    get_file_stem, is_markdown_file, link_is_external, path_slash, path_to_str,
};

/// Given the path to the CSV file and the "name" - compute the path to the markdown file that
/// it should point to.
fn get_name_link_path(path: &Path, name: &str) -> PathBuf {
    let mut path = path.to_path_buf();
    path.set_extension("");
    path.push(name);
    path.set_extension("md");
    path
}

fn write_headers<T: Write, U: Read>(
    reader: &mut csv::Reader<U>,
    writer: &mut BufWriter<T>,
) -> Result<Vec<String>> {
    let headers = reader.headers()?;
    let mut header_lengths: Vec<usize> = Vec::new();
    let mut header_strings: Vec<String> = Vec::new();

    for (_, header) in headers.iter().enumerate() {
        write!(writer, "|{header}")?;
        header_lengths.push(header.len());
        header_strings.push(String::from(header))
    }

    writeln!(writer, "|")?;
    for header_length in header_lengths {
        write!(writer, "| {} ", "-".repeat(header_length))?;
    }
    writeln!(writer, "|")?;
    Ok(header_strings)
}

fn write_field<T: Write>(writer: &mut BufWriter<T>, field: &str) -> Result<()> {
    let field = if field.is_empty() { " " } else { field };
    write!(writer, "| {field} ")?;
    Ok(())
}

/// The cell value is a comma-separated list of files.  We need to turn these into links if possible.
fn write_files<T: Write>(writer: &mut BufWriter<T>, field: &str, index: &Index) -> Result<()> {
    if field.is_empty() {
        // If the field has nothing in it, then pass through.
        write_field(writer, "")?;
        return Ok(());
    }

    debug!("Writing file links for: {field}");
    // Parse the field - it should be a comma-separated list.
    write!(writer, "| ")?;
    for f in field.split(",").map(|x| x.trim()) {
        // For external links, just pass them through.
        if link_is_external(f) {
            writer.write_all(f.as_bytes())?;
            continue;
        }
        // Decode the file name, turn it into a path.
        let f = urlencoding::decode(f)?.to_string();
        let path = Path::new(&f);
        if let Some(elem) = index.find_by_path(path) {
            let slash = path_slash(&elem.new_path)?;
            let link = fmt_wiki_link(&slash, None);
            writer.write_all(link.as_bytes())?;
        } else {
            info!("File not found: {f}");
            // Otherwise, just print it out.
            writer.write_all(f.as_bytes())?;
        }
    }
    Ok(())
}

fn write_name_link<T: Write>(
    writer: &mut BufWriter<T>,
    field: &str,
    new_path: &Path,
    index: &Index,
) -> Result<()> {
    // Compute the potential link destination based on the path of the CSV file and the
    // value found in the "Name" column.
    let path = get_name_link_path(new_path, field);
    if let Some(elem) = index.find_by_output_path(&path) {
        // If a markdown file was found, then write the link.
        let link_addr = elem.new_path.as_path();
        let addr = path_slash(link_addr)?;
        let text = get_file_stem(link_addr)?;
        let f = fmt_wiki_link(&addr, empty_is_none(&text));
        write_field(writer, &f)?;
    } else {
        // The name link wasn't found, so just write it out as a string.
        info!("Link not found: {field}");
        write_field(writer, field)?;
    }
    Ok(())
}

pub fn convert_csv_to_markdown(paths: &index::Paths, index: &Index) -> Result<()> {
    let input = paths.input_path();
    let output = paths.output_path();
    let new_path = paths.new_path.as_path();

    let mut reader = csv::Reader::from_path(&input)?;
    let mut writer = open_output_file(&output)?;

    // First, write the headers.
    let headers = write_headers(&mut reader, &mut writer)?;

    for row in reader.records() {
        let row = row?;
        // Iterate through the cells of the row, with the header of each cell.
        for cell_with_header in row.iter().enumerate().zip(
            // Join the 'column/cell' pair with...
            headers.iter().map(|h| h.as_str()), // Turn the headers into slices.
        ) {
            let ((column, field), header) = cell_with_header;
            if column == 0 && header == "Name" {
                write_name_link(&mut writer, field, new_path, index)?;
            } else if header == "Files" {
                write_files(&mut writer, field, index)?;
            } else {
                write_field(&mut writer, field)?;
            }
        }
        writeln!(writer, "|")?;
    }

    writeln!(writer)?;
    writeln!(writer)?;
    writeln!(writer, "----")?;
    writeln!(writer, "#notion2obsidian #csvimport")?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_name_link_path() {
        let new_path = Path::new("Appliances.csv");

        // CSV files can have links to markdown documents in the *folder* with the same name.

        // If the CSV "Appliances.csv" has a Name column value of "Refrigerator", then the
        // document should be "Appliances/Refrigerator.md".

        let result = get_name_link_path(new_path, "Refrigerator");
        assert_eq!(result, Path::new("Appliances/Refrigerator.md"));
    }
}
