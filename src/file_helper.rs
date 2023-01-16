use std::fs::{create_dir_all, File};
use anyhow::{Context, Result};
use std::path::Path;
use std::io::{BufRead, BufReader, BufWriter, Write};
use log::{debug, warn};

pub fn create_parent_if_needed(outpath: &Path) -> Result<()> {
    if let Some(p) = outpath.parent() {
        create_if_needed(p)?
    }
    Ok(())
}

pub fn create_if_needed(p: &Path) -> Result<()> {
    if !p.exists() {
        debug!("Creating directory {}", p.display());
        create_dir_all(p).with_context(|| { format!("Unable to create directory {}", p.display()) })?;
    }
    Ok(())
}

/// Open the specified path for output, creating the parent directory if needed.
pub fn open_output_file(output_path: &Path) -> Result<BufWriter<File>> {
    // Open the output file, create parent directory first.
    create_parent_if_needed(output_path)?;
    let output = File::create(output_path)?;
    let writer = BufWriter::new(output);
    Ok(writer)
}

pub fn process_lines<F>(input_path: &Path, output_path: &Path, mut line_processor: F) -> Result<()>
    where F: FnMut(&str) -> Result<String> {

    // Open the input file.
    let input = File::open(input_path).with_context(
        || { format!("Could not open {} for input", input_path.display()) })?;
    let reader = BufReader::new(input);

    let mut writer = open_output_file(output_path)?;

    // Process each line, replacing links.
    for line in reader.lines().filter_map(|ln| ln.ok()) {
        let processed_line = line_processor(&line)?;
        writeln!(writer, "{}", processed_line)?;
    }
    writer.flush()?;
    Ok(())
}

/// Copies the file, log a warning if there was a problem.
pub fn copy_file(input_path: &Path, output_path: &Path) -> u64 {
    let r = std::fs::copy(input_path, output_path);
    match r
    {
        Ok(n) => n,
        Err(e) => {
            warn!("Error copying [{}] to [{}]: {}",
                input_path.display(), output_path.display(), e);
            0   // We didn't copy anything.
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::remove_file;
    use super::*;

    #[test]
    fn test_process_lines() {
        let input = Path::new("test-data/test.md");
        assert!(input.exists());
        let output = Path::new("target/test-data/test-out.md");
        if output.exists() {
            remove_file(output).unwrap();
        }
        assert_eq!(output.exists(), false);

        let mut count = 0;
        let result = process_lines(input, output, |line| {
            count = count + 1;
            Ok(String::from(line))
        });
        assert!(result.is_ok());
        assert_eq!(count, 9);
    }
}