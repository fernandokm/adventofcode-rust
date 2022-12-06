use std::{
    borrow::Cow,
    hash::Hash,
    marker::PhantomData,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;
use rustc_hash::FxHashMap;

use crate::ProblemId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Spec {
    pub id: ProblemId,
    pub variant: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No input available for ({}/{}/{})", spec.id.year, spec.id.day, spec.variant)]
    NoInputAvailable { spec: Spec },

    #[error("Invalid input file name: {filename}")]
    InvalidFileName { filename: String },

    #[error("Invalid input file encoding: {0}")]
    InvalidFileEncodingInStr(#[from] std::str::Utf8Error),

    #[error("Invalid input file encoding: {0}")]
    InvalidFileEncodingInString(#[from] std::string::FromUtf8Error),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid year \"{year}\" in file \"{filename}\": {source}")]
    InvalidYear {
        filename: String,
        year: String,
        source: <u32 as FromStr>::Err,
    },

    #[error("Invalid day \"{day}\" in file \"{filename}\": {source}")]
    InvalidDay {
        filename: String,
        day: String,
        source: <u32 as FromStr>::Err,
    },

    #[error("Duplicate input files for ({}/{}/{}): {filename1} and {filename2}", spec.id.year, spec.id.day, spec.variant)]
    DuplicateInputs {
        filename1: String,
        filename2: String,
        spec: Spec,
    },
}

type Result<T> = std::result::Result<T, Error>;

pub fn parse_input_filename(filename: &str) -> Result<Spec> {
    lazy_static! {
        static ref FILENAME_RE: Regex =
            Regex::new("^(?:.*/)?([0-9]+)/day([0-9]+)_([^\\.]+)\\.txt$").unwrap();
    }

    let captures = FILENAME_RE
        .captures(filename)
        .ok_or(Error::InvalidFileName {
            filename: filename.to_owned(),
        })?;

    let year = captures.get(1).unwrap().as_str();
    let year: u32 = year.parse().map_err(|source| Error::InvalidYear {
        filename: filename.to_owned(),
        year: year.to_owned(),
        source,
    })?;

    let day = captures.get(2).unwrap().as_str();
    let day: u32 = day.parse().map_err(|source| Error::InvalidDay {
        filename: filename.to_owned(),
        day: day.to_owned(),
        source,
    })?;

    let variant = captures.get(3).unwrap().as_str();
    let id = ProblemId { year, day };
    Ok(Spec {
        id,
        variant: variant.to_owned(),
    })
}

pub trait Source {
    fn get(&self, key: &Spec) -> Result<Cow<'_, str>>;
    // TODO: there are more efficient ways to do this
    fn keys(&self) -> Vec<&Spec>;
}

pub fn from_embedded<E: RustEmbed>() -> Result<EmbeddedSource<E>> {
    EmbeddedSource::new()
}

pub fn from_fs(path: impl AsRef<Path>) -> Result<FSSource> {
    FSSource::new(path)
}

pub fn from_file(spec: Spec, file_path: impl AsRef<Path>) -> FSSource {
    FSSource::with_file(spec, file_path)
}

#[must_use]
pub fn new_memory<'a>() -> MemorySource<'a> {
    MemorySource::new()
}

pub fn chain<I1: Source, I2: Source>(first: I1, second: I2) -> ChainedSource<I1, I2> {
    ChainedSource::new(first, second)
}

pub struct EmbeddedSource<E: RustEmbed> {
    embed: PhantomData<*const E>,
    file_paths: FxHashMap<Spec, Cow<'static, str>>,
}

impl<E: RustEmbed> EmbeddedSource<E> {
    pub fn new() -> Result<Self> {
        let mut file_paths: FxHashMap<Spec, Cow<'static, str>> = FxHashMap::default();
        for file_path in E::iter() {
            let spec = parse_input_filename(&file_path)?;
            if let Some(old) = file_paths.insert(spec, file_path) {
                // If we have a duplicate entry, recompute the spec (it was moved in the line
                // above), recover the file path that was inserted into the map,
                // and return an error.
                let spec = parse_input_filename(&old).unwrap();
                let file_path = file_paths.remove(&spec).unwrap();
                return Err(Error::DuplicateInputs {
                    filename1: file_path.into_owned(),
                    filename2: old.into_owned(),
                    spec,
                });
            }
        }

        let embed = PhantomData;
        Ok(EmbeddedSource { embed, file_paths })
    }
}

impl<E: RustEmbed> Source for EmbeddedSource<E> {
    fn get(&self, spec: &Spec) -> Result<Cow<'_, str>> {
        let file = self
            .file_paths
            .get(spec)
            .and_then(|file_path| E::get(file_path))
            .ok_or(Error::NoInputAvailable { spec: spec.clone() })?;

        let content = match file.data {
            Cow::Borrowed(data) => Cow::Borrowed(std::str::from_utf8(data)?),
            Cow::Owned(data) => Cow::Owned(String::from_utf8(data)?),
        };
        Ok(content)
    }

    fn keys(&self) -> Vec<&Spec> {
        self.file_paths.keys().collect()
    }
}

#[derive(Default)]
pub struct FSSource {
    file_paths: FxHashMap<Spec, PathBuf>,
}

impl FSSource {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let mut source = FSSource::default();
        let path = path.as_ref();
        if path.is_file() {
            source.add_path(path.to_path_buf())?;
        } else {
            for entry in std::fs::read_dir(path)? {
                source.add_path(entry?.path())?;
            }
        }

        Ok(source)
    }

    pub fn with_file(spec: Spec, file_path: impl AsRef<Path>) -> Self {
        let mut source = FSSource::default();
        source.add_path_with_spec(spec, file_path.as_ref().to_path_buf());
        source
    }

    pub fn add_path(&mut self, path: PathBuf) -> Result<()> {
        let path_str = std::str::from_utf8(path.as_os_str().as_bytes())?;
        let spec = parse_input_filename(path_str)?;
        self.add_path_with_spec(spec, path);
        Ok(())
    }

    pub fn add_path_with_spec(&mut self, spec: Spec, path: PathBuf) {
        self.file_paths.insert(spec, path);
    }
}

impl Source for FSSource {
    fn get(&self, spec: &Spec) -> Result<Cow<'_, str>> {
        let file_path = self
            .file_paths
            .get(spec)
            .ok_or(Error::NoInputAvailable { spec: spec.clone() })?;

        let content = std::fs::read_to_string(file_path)?;
        Ok(Cow::Owned(content))
    }

    fn keys(&self) -> Vec<&Spec> {
        self.file_paths.keys().collect()
    }
}

pub struct MemorySource<'a> {
    files: FxHashMap<Spec, Cow<'a, str>>,
}

impl<'a> MemorySource<'a> {
    #[must_use]
    pub fn new() -> Self {
        MemorySource {
            files: FxHashMap::default(),
        }
    }

    #[must_use]
    pub fn from(files: FxHashMap<Spec, Cow<'a, str>>) -> Self {
        MemorySource { files }
    }

    pub fn add_cow(&mut self, spec: Spec, content: Cow<'a, str>) {
        self.files.insert(spec, content);
    }

    pub fn add_str(&mut self, spec: Spec, content: &'a str) {
        self.add_cow(spec, Cow::Borrowed(content));
    }

    pub fn add_string(&mut self, spec: Spec, content: String) {
        self.add_cow(spec, Cow::Owned(content));
    }
}

impl<'a> Default for MemorySource<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Source for MemorySource<'a> {
    fn get(&self, spec: &Spec) -> Result<Cow<'_, str>> {
        let content = self
            .files
            .get(spec)
            .ok_or(Error::NoInputAvailable { spec: spec.clone() })?;
        Ok(Cow::Borrowed(content))
    }

    fn keys(&self) -> Vec<&Spec> {
        self.files.keys().collect()
    }
}

pub struct ChainedSource<I1: Source, I2: Source> {
    first: I1,
    second: I2,
}

impl<I1: Source, I2: Source> ChainedSource<I1, I2> {
    pub fn new(first: I1, second: I2) -> Self {
        ChainedSource { first, second }
    }
}

impl<I1: Source, I2: Source> Source for ChainedSource<I1, I2> {
    fn get(&self, spec: &Spec) -> Result<Cow<'_, str>> {
        match self.first.get(spec) {
            Err(Error::NoInputAvailable { .. }) => self.second.get(spec),
            other => other,
        }
    }

    fn keys(&self) -> Vec<&Spec> {
        let mut keys = self.first.keys();
        keys.append(&mut self.second.keys());
        keys
    }
}
