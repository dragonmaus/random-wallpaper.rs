use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

pub(crate) fn get() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Err(Box::new(Error::new(
        ErrorKind::Unsupported,
        "wallpaper operations are not yet supported on this platform",
    )))
}

pub(crate) fn set(_path: &Path) -> program::Result {
    Err(Box::new(Error::new(
        ErrorKind::Unsupported,
        "wallpaper operations are not yet supported on this platform",
    )))
}
