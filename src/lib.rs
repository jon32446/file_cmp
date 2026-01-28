use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

/// Default buffer size for file comparison (64KB)
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// Parses a chunk size string (e.g., "4k", "2M", "128K") into bytes.
///
/// Supports suffixes: k/K (kilobytes), m/M (megabytes), g/G (gigabytes).
/// Returns None if the string is invalid.
pub fn parse_chunk_size(s: &str) -> Option<usize> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num_str, multiplier) = match s.chars().last()? {
        'k' | 'K' => (&s[..s.len() - 1], 1024),
        'm' | 'M' => (&s[..s.len() - 1], 1024 * 1024),
        'g' | 'G' => (&s[..s.len() - 1], 1024 * 1024 * 1024),
        '0'..='9' => (s, 1),
        _ => return None,
    };

    num_str.parse::<usize>().ok().map(|n| n * multiplier)
}

#[derive(Debug, Eq, PartialEq)]
pub enum FileDiff {
    Equal,
    Different(usize),
    LeftOnly,
    RightOnly,
    Error(String),
}

impl FileDiff {
    pub fn as_number(&self) -> String {
        match self {
            Self::Equal => "-1".to_string(),
            Self::Different(d) => format!("{}", d),
            Self::LeftOnly => "-2".to_string(),
            Self::RightOnly => "-3".to_string(),
            Self::Error(_) => "-4".to_string(),
        }
    }

    pub fn as_desc(&self) -> &'static str {
        match self {
            Self::Equal => "equal",
            Self::Different(_) => "diff",
            Self::LeftOnly => "left only",
            Self::RightOnly => "right only",
            Self::Error(_) => "error",
        }
    }
}

pub fn is_dir<P: AsRef<Path>>(path1: P) -> io::Result<bool> {
    let file1_meta = fs::metadata(&path1)?;
    Ok(file1_meta.is_dir())
}

pub fn compare_files<P: AsRef<Path>>(
    path1: P,
    path2: P,
    quick: bool,
    chunk_size: usize,
) -> io::Result<FileDiff> {
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

    let mut buffer1 = vec![0u8; chunk_size];
    let mut buffer2 = vec![0u8; chunk_size];
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

pub fn compare_dirs<P: AsRef<Path>>(
    dir1: P,
    dir2: P,
    quick: bool,
    chunk_size: usize,
) -> io::Result<Vec<(PathBuf, FileDiff)>> {
    let mut results = vec![];

    for entry in fs::read_dir(&dir1)? {
        let entry = entry?;
        let path = entry.path();

        let filename = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        if path.is_dir() {
            let other_path = dir2.as_ref().join(filename);
            if other_path.is_dir() {
                results.extend(compare_dirs(&path, &other_path, quick, chunk_size)?);
            } else {
                results.push((path, FileDiff::LeftOnly));
            }
        } else {
            let other_path = dir2.as_ref().join(filename);
            if other_path.exists() {
                match compare_files(&path, &other_path, quick, chunk_size) {
                    Ok(result) => results.push((path, result)),
                    Err(e) => results.push((path, FileDiff::Error(e.to_string()))),
                }
            } else {
                results.push((path, FileDiff::LeftOnly));
            }
        }
    }

    for entry in fs::read_dir(&dir2)? {
        let entry = entry?;
        let path = entry.path();

        let filename = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        if path.is_dir() {
            let other_path = dir1.as_ref().join(filename);
            if other_path.is_dir() {
                results.extend(compare_dirs(&other_path, &path, quick, chunk_size)?);
            } else {
                results.push((path, FileDiff::RightOnly));
            }
        } else {
            let other_path = dir1.as_ref().join(filename);
            if !other_path.exists() {
                results.push((path, FileDiff::RightOnly));
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chunk_size_kilobytes() {
        assert_eq!(parse_chunk_size("4k"), Some(4 * 1024));
        assert_eq!(parse_chunk_size("128K"), Some(128 * 1024));
    }

    #[test]
    fn test_parse_chunk_size_megabytes() {
        assert_eq!(parse_chunk_size("2m"), Some(2 * 1024 * 1024));
        assert_eq!(parse_chunk_size("16M"), Some(16 * 1024 * 1024));
    }

    #[test]
    fn test_parse_chunk_size_gigabytes() {
        assert_eq!(parse_chunk_size("1g"), Some(1024 * 1024 * 1024));
        assert_eq!(parse_chunk_size("2G"), Some(2 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_parse_chunk_size_bytes() {
        assert_eq!(parse_chunk_size("4096"), Some(4096));
        assert_eq!(parse_chunk_size("65536"), Some(65536));
    }

    #[test]
    fn test_parse_chunk_size_invalid() {
        assert_eq!(parse_chunk_size(""), None);
        assert_eq!(parse_chunk_size("abc"), None);
        assert_eq!(parse_chunk_size("4x"), None);
    }
}
