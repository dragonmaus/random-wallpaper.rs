use std::{
    ffi::OsString,
    io::Error,
    iter::once,
    os::windows::{ffi::OsStrExt, prelude::*},
    path::{Path, PathBuf},
    result::Result,
};

use winapi::{
    shared::minwindef::TRUE,
    um::{
        winnt::{PVOID, WCHAR},
        winuser::{
            SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETDESKWALLPAPER,
            SPI_SETDESKWALLPAPER,
        },
    },
};

const MAXPATH: usize = winapi::shared::minwindef::MAX_PATH;

// shamelessly stolen from https://github.com/kiedtl/painter
pub(crate) fn get() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let imgpath = [0 as WCHAR; MAXPATH];
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            MAXPATH as u32,
            imgpath.as_ptr() as PVOID,
            0,
        )
    };
    let strpos = imgpath.iter().position(|&a| a == 0).unwrap();
    let path = OsString::from_wide(&imgpath[..strpos]);
    match ret {
        TRUE => Ok(path.into()),
        _ => Err(Error::last_os_error().into()),
    }
}

pub(crate) fn set(path: &Path) -> program::Result {
    let mut wpath: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<_>>();
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wpath.as_mut_ptr() as PVOID,
            SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
        )
    };
    match ret {
        TRUE => Ok(0),
        _ => Err(Error::last_os_error().into()),
    }
}
