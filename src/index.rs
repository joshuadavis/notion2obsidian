use std::collections::HashMap;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::slice::Iter;
use log::info;
use walkdir::WalkDir;

/// Represents the file extension (file type) for a particular entry.
#[derive(Eq, PartialEq, Debug)]
pub enum Ext {
    Markdown,
    Table,
    Other
}

fn as_str(osstr: Option<&std::ffi::OsStr>) -> &str {
    osstr.unwrap_or_default().to_str().unwrap_or_default()
}

impl Ext {
    fn from_path(path: &Path) -> Self {
        match as_str(path.extension()).to_lowercase().as_str() {
            "md" => Ext::Markdown,
            "csv" => Ext::Table,
            _ => Ext::Other
        }
    }
}

/// Represents a file in the index.
#[derive(Debug)]
pub struct Element {
    /// The extension, or type, of the input file.
    pub ext: Ext,
    /// The path relative to the base input directory.
    pub path: PathBuf,
    /// The new path for the output file.  Relative to the output directory.
    pub output_path: PathBuf,
}

/// Computes the output path for a given input path.
fn compute_new_path(path: &Path) -> Result<PathBuf> {
    let mut buf = PathBuf::new();
    // Assume that the given path has been made relative to the base directory already.
    for component in path.components() {
        let new = crate::rex::replace_hex(component.as_os_str())?;
        buf.push(Path::new(&new));
    }
    // Replace the extension for some files.
    if Ext::from_path(&buf) == Ext::Table {
        buf.set_extension("md");
    }
    Ok(buf)
}

impl Element {
    fn new(input_path: &Path, base_dir: &Path) -> Result<Self> {
        let path = input_path.strip_prefix(base_dir)?;
        Ok(Self {
            ext: Ext::from_path(input_path),
            path: path.to_path_buf(),
            output_path: compute_new_path(path)?,
        })
    }
}

/// An index of all the input files, and some metadata about them.
#[derive(Debug)]
pub struct Index {
    /// All the elements in the input.  Using Rc because we're going to reference these elements
    /// from a HashMap.
    elements: Vec<Rc<Element>>,
    /// A map of the elements, by their original path (relative to the base directory).
    by_path: HashMap<PathBuf, Rc<Element>>,
    /// A map of the elements, by their output path (relative to the output directory).
    by_output_path: HashMap<PathBuf, Rc<Element>>,
}

/// Determines which input paths should be indexed/processed.
fn should_process(path: &Path) -> bool {
    path.is_file() && !as_str(path.file_name()).starts_with(".")
}

impl Index {

    /// Index the given directory.
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let mut elements: Vec<Rc<Element>> = Vec::new();
        let mut by_path : HashMap<PathBuf, Rc<Element>> = HashMap::new();
        let mut by_output_path : HashMap<PathBuf, Rc<Element>> = HashMap::new();
        for entry in WalkDir::new(dir) {
            let entry = entry?; // unwrap the Result, hang on to it as a local var to give it a lifetime.
            let path = entry.path();
            if should_process(path) {
                info!("Processing {}", path.display());
                let elem_rc = Rc::new(
                    Element::new(path, dir)?);
                // Okay, so now we have the Rc.  Clone it first, to get the reference into the map.
                by_path.insert(elem_rc.path.clone(), elem_rc.clone());
                by_output_path.insert(elem_rc.output_path.clone(), elem_rc.clone());
                // Then consume it (move it) into the vector.
                elements.push(elem_rc);
            }
        }
        Ok( Self { elements, by_path, by_output_path })
    }

    /// Find a file given it's original path.
    pub fn find_by_path(&self, path: &Path) -> Option<&Rc<Element>> {
        self.by_path.get(path)
    }

    /// Find a file given it's output path.
    pub fn find_by_output_path(&self, path: &Path) -> Option<&Rc<Element>> {
        self.by_output_path.get(path)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn iter(&self) -> Iter<'_, Rc<Element>> {
        self.elements.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_new_path() {
        let path = Path::new("Home Computing d0f2033b0e554288a4310335468648ad/Test It Out 080195c4abc7405982e5dff80da8499c.md");
        let buf = compute_new_path(path).unwrap();
        assert_eq!(buf, Path::new("Home Computing/Test It Out.md"));
        let path = Path::new("Home Computing d0f2033b0e554288a4310335468648ad/Test Table 080195c4abc7405982e5dff80da8499c.csv");
        let buf = compute_new_path(path).unwrap();
        assert_eq!(buf, Path::new("Home Computing/Test Table.md"));
    }

    #[test]
    fn test_index() {
        let index = Index::from_dir(Path::new("test-data/My Links 4d87e5fbcac64818adbd9511585bd720")).unwrap();
        assert_eq!(index.len(), 28);
        let elem = index.find_by_path(Path::new("How to Setup a DNS Server for a Home Lab on Ubuntu bfee0474ac0345ab9c6fcce76ac20d63/Untitled Database 240461ceab7040889b2171196ea67c0d.csv"));
        assert!(elem.is_some());
        let elem = elem.unwrap();
        assert_eq!(elem.ext, Ext::Table);
    }
}