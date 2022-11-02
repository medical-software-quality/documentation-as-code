use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

pub fn list_directory<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, String> {
    let path1 = path.as_ref().to_str().unwrap().to_string();

    let paths = fs::read_dir(path).map_err(|_| format!("Can't open directory {}", path1))?;

    paths
        .map(|path| path.map(|path| path.path()).map_err(|e| e.to_string()))
        .collect()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
    let path1 = path.as_ref().to_str().unwrap().to_string();

    let mut file = std::fs::File::open(path).map_err(|_| format!("Can't open {}", path1))?;

    let mut chunk = vec![];
    file.read_to_end(&mut chunk)
        .map_err(|_| format!("Can't read {}", path1))?;
    String::from_utf8(chunk).map_err(|_| "File is not valid utf8".to_string())
}
