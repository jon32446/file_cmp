use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

pub fn is_dir<P: AsRef<Path>>(path1: P) -> io::Result<bool> {
    let file1_meta = fs::metadata(&path1)?;
    Ok(file1_meta.is_dir())
}

pub fn compare_files<P: AsRef<Path>>(path1: P, path2: P, quick: bool) -> io::Result<Option<usize>> {
    let file1_meta = fs::metadata(&path1)?;
    let file2_meta = fs::metadata(&path2)?;

    if file1_meta.len() == 0 || file2_meta.len() == 0 {
        return Ok(Some(0));
    }

    if quick && file1_meta.len() != file2_meta.len() {
        return Ok(Some(0));
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
            return Ok(None);
        }

        if buffer1[..len1] != buffer2[..len2] {
            if quick {
                return Ok(Some(0));
            }
            for i in 0..len1 {
                if buffer1[i] != buffer2[i] {
                    return Ok(Some(pos + i));
                }
            }
        }

        pos += len1;
    }
}

pub fn compare_dirs<P: AsRef<Path>>(dir1: P, dir2: P, quick: bool) -> Vec<(PathBuf, i64)> {
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
                    Ok(None) => results.push((path, -1)),
                    Ok(Some(offset)) => results.push((
                        path,
                        offset.try_into().expect("Failed to convert offset value"),
                    )),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                results.push((path, -2));
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
                results.push((path, -3));
            }
        }
    }

    results
}
