use std::env;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use env_logger::Builder;
use log::{debug, info};

use crate::file_helper::{create_if_needed, create_parent_if_needed};
use crate::index::{Ext, Paths};

mod extract_zip;
mod file_helper;
mod index;
mod markdown;
mod path_helper;
mod rex;
mod table;

fn main() -> Result<()> {
    // Initialize logging.
    {
        let mut builder = Builder::from_default_env();
        builder.filter_level(log::LevelFilter::Info).try_init()?;
    }

    info!("notion2obsidian starting...");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments!"));
    }
    let input_str = &args[1];
    let input_path = Path::new(input_str);
    if !input_path.exists() {
        return Err(anyhow!("input {} does not exist!", input_str));
    }

    info!("Input: {}", input_path.display());

    // If the input file ends with ".zip", then extract it to a directory.
    let dir: PathBuf = if input_path.is_file() && path_helper::is_zip_file(input_path)? {
        info!("Unzipping {}...", input_path.display());
        extract_zip::extract_zip(input_path)?
    } else {
        input_path.to_path_buf()
    };

    let output_dir = env::current_dir()?.join("output");
    info!("Output: {}", output_dir.display());
    if output_dir.exists() {
        info!("Removing existing output directory...");
        remove_dir_all(&output_dir)?;
    }
    create_if_needed(output_dir.as_path())?;

    // Build up a map of old path to new path, don't actually copy the files yet.
    info!("Building index...");
    let index = index::Index::from_dir(&dir)?;
    info!("Path map contains {} entries", index.len());

    // Walk through the map and copy the files.
    for (i, elem) in index.iter().enumerate() {
        let paths = Paths::from_elem(elem, &dir, &output_dir);
        debug!("[{}] {:?}", i, paths);
        create_parent_if_needed(paths.output_path().as_path())?;
        match elem.ext {
            Ext::Table => {
                // Convert CSV files to markdown table?
                table::convert_csv_to_markdown(&paths, &index)?;
            }
            Ext::Markdown => {
                // Process markdown.
                markdown::process_markdown(&paths, &index)?;
            }
            _ => {
                // Otherwise, just copy.
                // Helper function that gives some error context if the copy fails.
                file_helper::copy_file(paths.input_path().as_path(), paths.output_path().as_path());
            }
        }
    }

    Ok(())
}
