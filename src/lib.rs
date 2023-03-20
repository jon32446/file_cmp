use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::Path;

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

        if len1 != len2 || buffer1[..len1] != buffer2[..len2] {
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
