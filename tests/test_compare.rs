use std::io;

use file_cmp::compare_files;

fn p(p: &str) -> String {
    format!(".\\tests\\testfiles\\{}", p)
}

#[test]
fn test_compare_files_equal() -> io::Result<()> {
    // Test when files are equal
    let res = compare_files(p("test.txt"), p("test.txt"))?;
    assert_eq!(res, None);
    Ok(())
}

#[test]
fn test_compare_files_differ_beginning() -> io::Result<()> {
    // Test when files differ at the beginning
    let res = compare_files(p("west.txt"), p("test.txt"))?;
    assert_eq!(res, Some(0));
    let res = compare_files(p("test.txt"), p("west.txt"))?;
    assert_eq!(res, Some(0));
    Ok(())
}

#[test]
fn test_compare_files_differ_end() -> io::Result<()> {
    // Test when files differ at the end
    let res = compare_files(p("test.txt"), p("tesx.txt"))?;
    assert_eq!(res, Some(3));
    let res = compare_files(p("tesx.txt"), p("test.txt"))?;
    assert_eq!(res, Some(3));
    Ok(())
}

#[test]
fn test_compare_files_middle() -> io::Result<()> {
    // Test when files differ in the middle
    let res = compare_files(p("test.txt"), p("text.txt"))?;
    assert_eq!(res, Some(2));
    let res = compare_files(p("text.txt"), p("test.txt"))?;
    assert_eq!(res, Some(2));
    Ok(())
}

#[test]
fn test_compare_files_one_shorter() -> io::Result<()> {
    // Test when file1 is shorter than file2
    let res = compare_files(p("testing.txt"), p("test.txt"))?;
    assert_eq!(res, Some(4));
    let res = compare_files(p("test.txt"), p("testing.txt"))?;
    assert_eq!(res, Some(4));
    Ok(())
}

#[test]
fn test_compare_files_one_emtpy() -> io::Result<()> {
    // Test when file1 is empty
    let res = compare_files(p("emptyfile.txt"), p("test.txt"))?;
    assert_eq!(res, Some(0));
    let res = compare_files(p("test.txt"), p("emptyfile.txt"))?;
    assert_eq!(res, Some(0));
    Ok(())
}
