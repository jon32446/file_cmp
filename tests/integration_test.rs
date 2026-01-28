use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("file_cmp").unwrap()
}

// --- File comparison tests ---

#[test]
fn test_file_equal() {
    cmd()
        .args(["tests/testfiles/test.txt", "tests/testfiles/test.txt"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Files are equal"));
}

#[test]
fn test_file_different() {
    cmd()
        .args(["tests/testfiles/test.txt", "tests/testfiles/west.txt"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Files differ at byte 0"));
}

#[test]
fn test_file_different_middle() {
    cmd()
        .args(["tests/testfiles/test.txt", "tests/testfiles/text.txt"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Files differ at byte 2"));
}

#[test]
fn test_file_not_found() {
    cmd()
        .args(["tests/testfiles/test.txt", "nonexistent.txt"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Error"));
}

// --- Machine-readable output tests ---

#[test]
fn test_machine_readable_equal() {
    cmd()
        .args(["-m", "tests/testfiles/test.txt", "tests/testfiles/test.txt"])
        .assert()
        .success()
        .code(0)
        .stdout("-1");
}

#[test]
fn test_machine_readable_different() {
    cmd()
        .args(["-m", "tests/testfiles/test.txt", "tests/testfiles/text.txt"])
        .assert()
        .code(1)
        .stdout("2");
}

// --- Quick mode tests ---

#[test]
fn test_quick_mode_different_sizes() {
    cmd()
        .args(["-q", "-m", "tests/testfiles/test.txt", "tests/testfiles/testing.txt"])
        .assert()
        .code(1)
        .stdout("0");
}

#[test]
fn test_quick_mode_equal() {
    cmd()
        .args(["-q", "tests/testfiles/test.txt", "tests/testfiles/test.txt"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Files are equal"));
}

// --- Chunk size tests ---

#[test]
fn test_chunk_size_small() {
    cmd()
        .args(["-c", "64", "tests/testfiles/test.txt", "tests/testfiles/test.txt"])
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_chunk_size_with_suffix() {
    cmd()
        .args(["-c", "1k", "tests/testfiles/test.txt", "tests/testfiles/test.txt"])
        .assert()
        .success()
        .code(0);
}

// --- Directory comparison tests ---

#[test]
fn test_directory_comparison() {
    let temp = TempDir::new().unwrap();
    let dir1 = temp.path().join("dir1");
    let dir2 = temp.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    fs::write(dir1.join("same.txt"), "content").unwrap();
    fs::write(dir2.join("same.txt"), "content").unwrap();
    fs::write(dir1.join("diff.txt"), "content1").unwrap();
    fs::write(dir2.join("diff.txt"), "content2").unwrap();
    fs::write(dir1.join("left_only.txt"), "left").unwrap();
    fs::write(dir2.join("right_only.txt"), "right").unwrap();

    cmd()
        .args([dir1.to_str().unwrap(), dir2.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("equal"))
        .stdout(predicate::str::contains("diff"))
        .stdout(predicate::str::contains("left only"))
        .stdout(predicate::str::contains("right only"));
}

#[test]
fn test_directory_diffs_only() {
    let temp = TempDir::new().unwrap();
    let dir1 = temp.path().join("dir1");
    let dir2 = temp.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    fs::write(dir1.join("same.txt"), "content").unwrap();
    fs::write(dir2.join("same.txt"), "content").unwrap();
    fs::write(dir1.join("diff.txt"), "content1").unwrap();
    fs::write(dir2.join("diff.txt"), "content2").unwrap();

    cmd()
        .args(["-d", dir1.to_str().unwrap(), dir2.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("diff"))
        .stdout(predicate::str::contains("equal").not());
}

#[test]
fn test_directory_all_equal() {
    let temp = TempDir::new().unwrap();
    let dir1 = temp.path().join("dir1");
    let dir2 = temp.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    fs::write(dir1.join("file.txt"), "content").unwrap();
    fs::write(dir2.join("file.txt"), "content").unwrap();

    cmd()
        .args([dir1.to_str().unwrap(), dir2.to_str().unwrap()])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("equal"));
}

#[test]
fn test_directory_machine_readable() {
    let temp = TempDir::new().unwrap();
    let dir1 = temp.path().join("dir1");
    let dir2 = temp.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    fs::write(dir1.join("same.txt"), "content").unwrap();
    fs::write(dir2.join("same.txt"), "content").unwrap();

    cmd()
        .args(["-m", dir1.to_str().unwrap(), dir2.to_str().unwrap()])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("-1"))
        .stdout(predicate::str::contains("(").not());
}

// --- Error handling tests ---

#[test]
fn test_empty_directory() {
    let temp = TempDir::new().unwrap();
    let dir1 = temp.path().join("dir1");
    let dir2 = temp.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    cmd()
        .args([dir1.to_str().unwrap(), dir2.to_str().unwrap()])
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_nested_directories() {
    let temp = TempDir::new().unwrap();
    let dir1 = temp.path().join("dir1");
    let dir2 = temp.path().join("dir2");
    fs::create_dir_all(dir1.join("subdir")).unwrap();
    fs::create_dir_all(dir2.join("subdir")).unwrap();

    fs::write(dir1.join("subdir").join("file.txt"), "content").unwrap();
    fs::write(dir2.join("subdir").join("file.txt"), "content").unwrap();

    cmd()
        .args([dir1.to_str().unwrap(), dir2.to_str().unwrap()])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("equal"));
}
