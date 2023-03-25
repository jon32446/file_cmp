use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, PartialEq)]
pub enum FileDiff {
    Equal,
    Different(usize),
    LeftOnly,
    RightOnly,
}

impl FileDiff {
    pub fn as_number(&self) -> String {
        match self {
            Self::Equal => "-1".to_string(),
            Self::Different(d @ _) => format!("{}", d),
            Self::LeftOnly => "-2".to_string(),
            Self::RightOnly => "-3".to_string(),
        }
    }

    pub fn as_desc(&self) -> &'static str {
        match self {
            Self::Equal => "equal",
            Self::Different(_) => "diff",
            Self::LeftOnly => "left only",
            Self::RightOnly => "right only",
        }
    }
}

pub fn is_dir<P: AsRef<Path>>(path1: P) -> io::Result<bool> {
    let file1_meta = fs::metadata(&path1)?;
    Ok(file1_meta.is_dir())
}

pub fn compare_files<P: AsRef<Path>>(path1: P, path2: P, quick: bool) -> io::Result<FileDiff> {
    let file1_meta = fs::metadata(&path1)?;
    let file2_meta = fs::metadata(&path2)?;

    if file1_meta.len() == 0 || file2_meta.len() == 0 {
        return match file1_meta.len() == file2_meta.len() {
            true => Ok(FileDiff::Equal),
            false => Ok(FileDiff::Different(0)),
        };
    }

    if quick && file1_meta.len() != file2_meta.len() {
        return Ok(FileDiff::Different(0));
    }

    let mut file1 = BufReader::new(File::open(path1)?);
    let mut file2 = BufReader::new(File::open(path2)?);

    let mut buffer1 = [0; 4096];
    let mut buffer2 = [0; 4096];
    let mut pos = 0;

    loop {
        let len1 = file1.read(&mut buffer1)?;
        let len2 = file2.read(&mut buffer2)?;

        if len1 == 0 && len2 == 0 {
            return Ok(FileDiff::Equal);
        }

        if buffer1[..len1] != buffer2[..len2] {
            if quick {
                return Ok(FileDiff::Different(0));
            }
            for i in 0..len1 {
                if buffer1[i] != buffer2[i] {
                    return Ok(FileDiff::Different(pos + i));
                }
            }
        }

        pos += len1;
    }
}

pub fn compare_dirs<P: AsRef<Path>>(dir1: P, dir2: P, quick: bool) -> Vec<(PathBuf, FileDiff)> {
    let mut results = vec![];

    for entry in fs::read_dir(&dir1).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_dir() {
            let other_path = dir2
                .as_ref()
                .join(path.file_name().expect("Failed to get filename"));
            results.extend(compare_dirs(&path, &other_path, quick));
        } else {
            let other_path = dir2
                .as_ref()
                .join(path.file_name().expect("Failed to get filename"));
            if other_path.exists() {
                match compare_files(&path, &other_path, quick) {
                    Ok(result @ _) => results.push((path, result)),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                results.push((path, FileDiff::LeftOnly));
            }
        }
    }

    for entry in fs::read_dir(dir2).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            let other_path = dir1
                .as_ref()
                .join(path.file_name().expect("Failed to get filename"));
            results.extend(compare_dirs(&other_path, &path, quick));
        } else {
            let other_path = dir1
                .as_ref()
                .join(path.file_name().expect("Failed to get filename"));
            if !other_path.exists() {
                results.push((path, FileDiff::RightOnly));
            }
        }
    }

    results
}
