use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

enum CompareResult {
    Equal,
    Differ,
}

fn compare_files<P: AsRef<Path>>(file1_path: P, file2_path: P) -> io::Result<CompareResult> {
    let mut file1 = BufReader::new(File::open(file1_path)?);
    let mut file2 = BufReader::new(File::open(file2_path)?);

    let mut buffer1 = [0; 4096];
    let mut buffer2 = [0; 4096];

    loop {
        let len1 = file1.read(&mut buffer1)?;
        let len2 = file2.read(&mut buffer2)?;

        if len1 != len2 {
            return Ok(CompareResult::Differ);
        }

        if len1 == 0 {
            return Ok(CompareResult::Equal);
        }

        if buffer1[..len1] != buffer2[..len2] {
            return Ok(CompareResult::Differ);
        }
    }
}

fn main() {
    use CompareResult::{Differ, Equal};
    match compare_files("test1.txt", "test2.txt") {
        Ok(Equal) => println!("Files are equal"),
        Ok(Differ) => println!("Files differ"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
