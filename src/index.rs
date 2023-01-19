use crate::path_helper::{component_to_string, get_parent};
use anyhow::Result;
use lazy_static::lazy_static;
use log::{debug, warn};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::slice::Iter;
use walkdir::WalkDir;

/// Represents the file extension (file type) for a particular entry.
#[derive(Eq, PartialEq, Debug)]
pub enum Ext {
    Markdown,
    Table,
    Other,
}

fn as_str(osstr: Option<&std::ffi::OsStr>) -> &str {
    osstr.unwrap_or_default().to_str().unwrap_or_default()
}

impl Ext {
    fn from_path(path: &Path) -> Self {
        match as_str(path.extension()).to_lowercase().as_str() {
            "md" => Ext::Markdown,
            "csv" => Ext::Table,
            _ => Ext::Other,
        }
    }
}

/// Represents a file in the index.
#[derive(Debug)]
pub struct Element {
    /// The extension, or type, of the input file.
    pub ext: Ext,
    /// The path relative to the base input directory.
    pub old_path: PathBuf,
    /// The new path for the output file.  Relative to the output directory.
    pub new_path: PathBuf,
}

/// Computes the output path for a given input path.
fn compute_new_path(path: &Path) -> Result<PathBuf> {
    let mut buf = PathBuf::new();
    // Assume that the given path has been made relative to the base directory already.
    for component in path.components() {
        let new = crate::rex::replace_hex(component_to_string(&component)?.as_str())?;
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
            old_path: path.to_path_buf(),
            new_path: compute_new_path(path)?,
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

fn relative_path(path: &Path, base_dir: &Path) -> Result<PathBuf> {
    lazy_static! {
        static ref DOTDOT: PathBuf = PathBuf::from("..");
    }
    let dotdot = DOTDOT.as_path();
    Ok(if path.starts_with(dotdot) {
        let path = path.strip_prefix(dotdot)?;
        get_parent(base_dir)?.join(path)
    } else {
        base_dir.join(path)
    })
}

impl Index {
    /// Index the given directory.
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let mut elements: Vec<Rc<Element>> = Vec::new();
        let mut by_path: HashMap<PathBuf, Rc<Element>> = HashMap::new();
        let mut by_output_path: HashMap<PathBuf, Rc<Element>> = HashMap::new();
        for entry in WalkDir::new(dir) {
            let entry = entry?; // unwrap the Result, hang on to it as a local var to give it a lifetime.
            let path = entry.path();
            if should_process(path) {
                debug!("Processing {}", path.display());
                let elem_rc = Rc::new(Element::new(path, dir)?);
                // Okay, so now we have the Rc.  Clone it first, to get the reference into the map.
                by_path.insert(elem_rc.old_path.clone(), elem_rc.clone());
                by_output_path.insert(elem_rc.new_path.clone(), elem_rc.clone());
                // Then consume it (move it) into the vector.
                elements.push(elem_rc);
            }
        }
        Ok(Self {
            elements,
            by_path,
            by_output_path,
        })
    }

    /// Find a file given it's original path.
    pub fn find_by_path(&self, path: &Path) -> Option<&Rc<Element>> {
        self.by_path.get(path)
    }

    pub fn find_by_path_or_relative_path(
        &self,
        path: &Path,
        base_dir: &Path,
    ) -> Option<&Rc<Element>> {
        match self.find_by_path(path) {
            Some(elem) => Some(elem),
            None => {
                let relative = relative_path(path, base_dir);
                match relative {
                    Ok(relative) => self.find_by_path(&relative),
                    Err(e) => {
                        warn!(
                            "Could not find relative path for {} due to {}",
                            path.display(),
                            e
                        );
                        None
                    }
                }
            }
        }
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

/// Paths, used while processing a file.
#[derive(Debug)]
pub struct Paths {
    /// The relative path to the input file.
    pub old_path: PathBuf,

    /// The relative path to the output file.
    pub new_path: PathBuf,

    /// The base input directory.
    pub input_dir: PathBuf,

    /// The base output directory.
    pub output_dir: PathBuf,
}

impl Paths {
    /// Create a new Paths object from an index Element
    pub fn from_elem(elem: &Rc<Element>, input_dir: &Path, output_dir: &Path) -> Self {
        Self {
            old_path: elem.old_path.clone(),
            new_path: elem.new_path.clone(),
            input_dir: input_dir.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
        }
    }

    /// Returns the full input path for the input file.
    pub fn input_path(&self) -> PathBuf {
        self.input_dir.join(self.old_path.as_path())
    }

    /// Returns the full output path for the output file.
    pub fn output_path(&self) -> PathBuf {
        self.output_dir.join(self.new_path.as_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_path() {
        let base_dir = Path::new("/home/josh");
        let path = Path::new("Downloads");
        let relative = relative_path(path, base_dir).unwrap();
        assert_eq!(relative, Path::new("/home/josh/Downloads"));
        let relative = relative_path(Path::new("../Downloads"), base_dir).unwrap();
        assert_eq!(relative, Path::new("/home/Downloads"));
    }

    #[test]
    fn test_paths() {
        let elem = Rc::new(Element {
            ext: Ext::Markdown,
            old_path: PathBuf::from("foo/bar.md"),
            new_path: PathBuf::from("foo/bar.md"),
        });
        let paths = Paths::from_elem(&elem, Path::new("input"), Path::new("output"));
        assert_eq!(paths.input_path(), Path::new("input/foo/bar.md"));
        assert_eq!(paths.output_path(), Path::new("output/foo/bar.md"));
    }

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
        let index = Index::from_dir(Path::new(
            "test-data/My Links 4d87e5fbcac64818adbd9511585bd720",
        ))
        .unwrap();
        assert_eq!(index.len(), 28);
        let elem = index.find_by_path(Path::new("How to Setup a DNS Server for a Home Lab on Ubuntu bfee0474ac0345ab9c6fcce76ac20d63/Untitled Database 240461ceab7040889b2171196ea67c0d.csv"));
        assert!(elem.is_some());
        let elem = elem.unwrap();
        assert_eq!(elem.ext, Ext::Table);
    }
}
