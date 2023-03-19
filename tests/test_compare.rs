use std::io;

use file_cmp::compare_files;

#[test]
fn test_compare_files_equal() -> io::Result<()> {
    // Test when files are equal
    let res = compare_files("test.txt", "test.txt")?;
    assert_eq!(res, None);
    Ok(())
}

#[test]
fn test_compare_files_differ_beginning() -> io::Result<()> {
    // Test when files differ at the beginning
    let res = compare_files("west.txt", "test.txt")?;
    assert_eq!(res, Some(0));
    let res = compare_files("test.txt", "west.txt")?;
    assert_eq!(res, Some(0));
    Ok(())
}

#[test]
fn test_compare_files_differ_end() -> io::Result<()> {
    // Test when files differ at the end
    let res = compare_files("test.txt", "tesx.txt")?;
    assert_eq!(res, Some(3));
    let res = compare_files("tesx.txt", "test.txt")?;
    assert_eq!(res, Some(3));
    Ok(())
}

#[test]
fn test_compare_files_middle() -> io::Result<()> {
    // Test when files differ in the middle
    let res = compare_files("test.txt", "text.txt")?;
    assert_eq!(res, Some(2));
    let res = compare_files("text.txt", "test.txt")?;
    assert_eq!(res, Some(2));
    Ok(())
}

#[test]
fn test_compare_files_one_shorter() -> io::Result<()> {
    // Test when file1 is shorter than file2
    let res = compare_files("testing.txt", "test.txt")?;
    assert_eq!(res, Some(4));
    let res = compare_files("test.txt", "testing.txt")?;
    assert_eq!(res, Some(4));
    Ok(())
}

#[test]
fn test_compare_files_one_emtpy() -> io::Result<()> {
    // Test when file1 is empty
    let res = compare_files("emptyfile.txt", "test.txt")?;
    assert_eq!(res, Some(0));
    let res = compare_files("test.txt", "emptyfile.txt")?;
    assert_eq!(res, Some(0));
    Ok(())
}
