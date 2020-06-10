//! # s2tw
//! A simple tool for converting Simple Chinese to Traditional Chinese(TW).

use std::fs;
use std::path::Path;

pub fn try_delete<P: AsRef<Path>>(path: P) {
    if fs::remove_file(path.as_ref()).is_err() {}
}
