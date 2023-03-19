use file_cmp::compare_files;

fn main() {
    match compare_files("test.txt", "testing.txt") {
        Ok(None) => println!("Files are equal"),
        Ok(Some(pos)) => println!("Files differ at offset {}", pos),
        Err(e) => eprintln!("Error: {}", e),
    }
}
