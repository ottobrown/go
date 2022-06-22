use std::path::PathBuf;

use rfd::FileDialog;

/// Open a file dialog for a .sgf file
pub fn open_sgf() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Smart Game Format", &["sgf"])
        .pick_file()
}
