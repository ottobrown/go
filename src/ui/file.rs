use std::path::PathBuf;

pub fn open_sgf(path: &mut Option<PathBuf>) {
    let dialog = rfd::FileDialog::new()
        .add_filter("SGF", &["sgf"])
        .pick_file();
 
    if let Some(p) = dialog {
        *path = Some(p);
    }
}
