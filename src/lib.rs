use rayon::prelude::*;
use std::collections::HashSet;
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

/// Represents the result of comparing two files or directory entries.
///
/// # Variants
///
/// * `Equal` - Files are byte-identical
/// * `Different(usize)` - Files differ, with the byte offset of first difference
/// * `LeftOnly` - Entry exists only in the left/first directory
/// * `RightOnly` - Entry exists only in the right/second directory
/// * `Error(String)` - An error occurred during comparison
///
/// # Example
///
/// ```
/// use file_cmp::FileDiff;
///
/// let diff = FileDiff::Different(42);
/// assert_eq!(diff.as_desc(), "diff");
/// assert_eq!(diff.as_number(), "42");
/// ```
#[derive(Debug, Eq, PartialEq)]
pub enum FileDiff {
    Equal,
    Different(usize),
    LeftOnly,
    RightOnly,
    Error(String),
}

impl FileDiff {
    /// Returns a machine-readable numeric representation of the diff result.
    ///
    /// * `-1` for Equal
    /// * Byte offset (0+) for Different
    /// * `-2` for LeftOnly
    /// * `-3` for RightOnly
    /// * `-4` for Error
    pub fn as_number(&self) -> String {
        match self {
            Self::Equal => "-1".to_string(),
            Self::Different(d) => format!("{}", d),
            Self::LeftOnly => "-2".to_string(),
            Self::RightOnly => "-3".to_string(),
            Self::Error(_) => "-4".to_string(),
        }
    }

    /// Returns a human-readable description of the diff result.
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

/// Checks if a path refers to a directory.
///
/// # Arguments
///
/// * `path1` - Path to check
///
/// # Returns
///
/// * `Ok(true)` if the path is a directory
/// * `Ok(false)` if the path is a file
/// * `Err` if the path doesn't exist or cannot be accessed
///
/// # Example
///
/// ```no_run
/// use file_cmp::is_dir;
///
/// if is_dir("src").unwrap() {
///     println!("src is a directory");
/// }
/// ```
pub fn is_dir<P: AsRef<Path>>(path1: P) -> io::Result<bool> {
    let file1_meta = fs::metadata(&path1)?;
    Ok(file1_meta.is_dir())
}

/// Compares two files byte-by-byte for equality.
///
/// # Arguments
///
/// * `path1` - Path to the first file
/// * `path2` - Path to the second file
/// * `quick` - If true, returns immediately on first difference without finding exact byte offset
/// * `chunk_size` - Size of read buffer in bytes (use `DEFAULT_CHUNK_SIZE` for default)
///
/// # Returns
///
/// * `Ok(FileDiff::Equal)` if files are identical
/// * `Ok(FileDiff::Different(offset))` if files differ, with byte offset of first difference
/// * `Err` if files cannot be read
///
/// # Example
///
/// ```no_run
/// use file_cmp::{compare_files, FileDiff, DEFAULT_CHUNK_SIZE};
///
/// let result = compare_files("file1.txt", "file2.txt", false, DEFAULT_CHUNK_SIZE).unwrap();
/// match result {
///     FileDiff::Equal => println!("Files are identical"),
///     FileDiff::Different(offset) => println!("Files differ at byte {}", offset),
///     _ => unreachable!(),
/// }
/// ```
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
            // In quick mode, return immediately without finding exact byte offset
            if quick {
                return Ok(FileDiff::Different(0));
            }
            for i in 0..len1 {
                if buffer1[i] != buffer2[i] {
                    return Ok(FileDiff::Different(pos + i));
                }
            }
            // If lengths differ but content of min(len1, len2) is same,
            // the difference is at the shorter length
            return Ok(FileDiff::Different(pos + len1.min(len2)));
        }

        pos += len1;
    }
}

/// Compares two directories recursively using parallel processing.
///
/// Compares all files in both directories, recursively descending into subdirectories.
/// Files are compared in parallel using rayon for improved performance.
///
/// # Arguments
///
/// * `dir1` - Path to the first (left) directory
/// * `dir2` - Path to the second (right) directory
/// * `quick` - If true, uses quick comparison mode for files
/// * `chunk_size` - Size of read buffer for file comparisons
///
/// # Returns
///
/// A vector of tuples containing the file path and its comparison result.
/// Paths from dir1 are used for files that exist in both directories.
///
/// # Example
///
/// ```no_run
/// use file_cmp::{compare_dirs, FileDiff, DEFAULT_CHUNK_SIZE};
///
/// let results = compare_dirs("dir1", "dir2", false, DEFAULT_CHUNK_SIZE).unwrap();
/// for (path, diff) in results {
///     if diff != FileDiff::Equal {
///         println!("{}: {}", path.display(), diff.as_desc());
///     }
/// }
/// ```
pub fn compare_dirs<P: AsRef<Path>>(
    dir1: P,
    dir2: P,
    quick: bool,
    chunk_size: usize,
) -> io::Result<Vec<(PathBuf, FileDiff)>> {
    let dir1 = dir1.as_ref();
    let dir2 = dir2.as_ref();

    // Collect entries from both directories
    let entries1: Vec<_> = fs::read_dir(dir1)?
        .filter_map(|e| e.ok())
        .collect();
    let entries2: Vec<_> = fs::read_dir(dir2)?
        .filter_map(|e| e.ok())
        .collect();

    // Build a set of filenames from dir1 to track what's been processed
    let dir1_names: HashSet<_> = entries1
        .iter()
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    // Process dir1 entries in parallel
    let results1: Vec<(PathBuf, FileDiff)> = entries1
        .par_iter()
        .flat_map(|entry| {
            let path = entry.path();
            let filename = match path.file_name() {
                Some(name) => name,
                None => return vec![],
            };

            if path.is_dir() {
                let other_path = dir2.join(filename);
                if other_path.is_dir() {
                    match compare_dirs(&path, &other_path, quick, chunk_size) {
                        Ok(sub_results) => sub_results,
                        Err(e) => vec![(path, FileDiff::Error(e.to_string()))],
                    }
                } else {
                    vec![(path, FileDiff::LeftOnly)]
                }
            } else {
                let other_path = dir2.join(filename);
                if other_path.exists() {
                    match compare_files(&path, &other_path, quick, chunk_size) {
                        Ok(result) => vec![(path, result)],
                        Err(e) => vec![(path, FileDiff::Error(e.to_string()))],
                    }
                } else {
                    vec![(path, FileDiff::LeftOnly)]
                }
            }
        })
        .collect();

    // Process dir2 entries for right-only items (skip items already in dir1)
    let results2: Vec<(PathBuf, FileDiff)> = entries2
        .par_iter()
        .flat_map(|entry| {
            let path = entry.path();
            let filename = match path.file_name() {
                Some(name) => name,
                None => return vec![],
            };

            // Skip if already processed from dir1
            if let Some(name_str) = filename.to_str() {
                if dir1_names.contains(name_str) {
                    return vec![];
                }
            }

            if path.is_dir() {
                vec![(path, FileDiff::RightOnly)]
            } else {
                vec![(path, FileDiff::RightOnly)]
            }
        })
        .collect();

    let mut results = results1;
    results.extend(results2);
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
