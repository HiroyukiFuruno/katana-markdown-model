use crate::KmmError;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum MarkdownInput {
    Content { path: PathBuf, content: String },
    File { path: PathBuf },
}

impl MarkdownInput {
    pub fn from_content(path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self::Content {
            path: path.into(),
            content: content.into(),
        }
    }

    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        Self::File { path: path.into() }
    }

    pub(crate) fn into_parts(self) -> Result<(PathBuf, String), KmmError> {
        match self {
            Self::Content { path, content } => Ok((path, content)),
            Self::File { path } => Self::read_file(&path),
        }
    }

    fn read_file(path: &Path) -> Result<(PathBuf, String), KmmError> {
        let content = std::fs::read_to_string(path).map_err(|source| KmmError::ReadFile {
            path: path.to_path_buf(),
            source,
        })?;
        Ok((path.to_path_buf(), content))
    }
}
