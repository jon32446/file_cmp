use file_cmp::{compare_files, DEFAULT_CHUNK_SIZE};
use file_cmp::FileDiff::*;
use std::io;
use std::path::PathBuf;

fn p(name: &str) -> PathBuf {
    PathBuf::from("tests").join("testfiles").join(name)
}

#[test]
fn test_compare_files_equal() -> io::Result<()> {
    // Test when files are equal
    let res = compare_files(p("test.txt"), p("test.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Equal);
    Ok(())
}

#[test]
fn test_compare_files_differ_beginning() -> io::Result<()> {
    // Test when files differ at the beginning
    let res = compare_files(p("west.txt"), p("test.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(0));
    let res = compare_files(p("test.txt"), p("west.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(0));
    Ok(())
}

#[test]
fn test_compare_files_differ_end() -> io::Result<()> {
    // Test when files differ at the end
    let res = compare_files(p("test.txt"), p("tesx.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(3));
    let res = compare_files(p("tesx.txt"), p("test.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(3));
    Ok(())
}

#[test]
fn test_compare_files_middle() -> io::Result<()> {
    // Test when files differ in the middle
    let res = compare_files(p("test.txt"), p("text.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(2));
    let res = compare_files(p("text.txt"), p("test.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(2));
    Ok(())
}

#[test]
fn test_compare_files_one_shorter() -> io::Result<()> {
    // Test when file1 is shorter than file2
    let res = compare_files(p("testing.txt"), p("test.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(4));
    let res = compare_files(p("test.txt"), p("testing.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(4));
    Ok(())
}

#[test]
fn test_compare_files_one_emtpy() -> io::Result<()> {
    // Test when file1 is empty
    let res = compare_files(p("emptyfile.txt"), p("test.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(0));
    let res = compare_files(p("test.txt"), p("emptyfile.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(0));
    Ok(())
}

#[test]
fn test_compare_files_both_emtpy() -> io::Result<()> {
    // Test when file1 is empty
    let res = compare_files(p("emptyfile.txt"), p("emptyfile.txt"), false, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Equal);
    Ok(())
}

#[test]
fn test_compare_files_quick_mode() -> io::Result<()> {
    // Quick mode should return Different(0) without finding exact byte offset
    let res = compare_files(p("test.txt"), p("text.txt"), true, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(0));
    // But equal files should still be Equal
    let res = compare_files(p("test.txt"), p("test.txt"), true, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Equal);
    Ok(())
}

#[test]
fn test_compare_files_quick_mode_different_sizes() -> io::Result<()> {
    // Quick mode with different file sizes should return immediately
    let res = compare_files(p("test.txt"), p("testing.txt"), true, DEFAULT_CHUNK_SIZE)?;
    assert_eq!(res, Different(0));
    Ok(())
}

#[test]
fn test_compare_files_small_chunk() -> io::Result<()> {
    // Test with very small chunk size (1 byte)
    let res = compare_files(p("test.txt"), p("text.txt"), false, 1)?;
    assert_eq!(res, Different(2));
    Ok(())
}
