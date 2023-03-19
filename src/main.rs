use clap::Parser;
use file_cmp::compare_files;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path
    file1: String,
    /// File path
    file2: String,
}

fn main() {
    let args = Args::parse();

    match compare_files(&args.file1, &args.file2) {
        Ok(None) => println!("Files are equal"),
        Ok(Some(pos)) => println!("Files differ at offset {}", pos),
        Err(e) => eprintln!("Error: {}", e),
    }
}
