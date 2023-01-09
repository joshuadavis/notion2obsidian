use std::path::{Path, PathBuf};
use std::{fs, io};
use zip::ZipArchive;
use anyhow::anyhow;
use zip::read::ZipFile;
use crate::file_helper::create_parent_if_needed;

fn extract_file(mut file: &mut ZipFile, outpath: &PathBuf) -> anyhow::Result<u64> {
    create_parent_if_needed(outpath)?;
    let mut outfile = fs::File::create(&outpath)?;
    Ok(io::copy(&mut file, &mut outfile)?)
}

pub fn extract_zip(input_path: &Path) -> anyhow::Result<PathBuf> {
    let file = fs::File::open(input_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut first_dir : Option<PathBuf> = None;
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let outpath = match zip_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if first_dir == None {
            first_dir = Some(outpath.parent().ok_or(anyhow!("Unable to get parent!"))?.to_path_buf());
        }

        if (*zip_file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            extract_file(&mut zip_file, &outpath)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = zip_file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    first_dir.ok_or(anyhow!("Unable to get first_dir!"))
}
