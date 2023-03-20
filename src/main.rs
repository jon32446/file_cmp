use clap::Parser;
use file_cmp::compare_files;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// File path to first file to compare
    file1: String,
    /// File path to second file to compare
    file2: String,
    /// Optional flag to enable machine-readable output
    #[arg(short('m'), long("machine"))]
    machine_readable: bool,
    /// Optional flag to do faster comparison and not output first diff offset
    #[arg(short, long)]
    quick: bool,
    /// Optional parameter to set the chunk size for reading the files, e.g. 4k, 2M
    #[arg(short, long)]
    chunk_size: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match compare_files(&args.file1, &args.file2, args.quick) {
        Ok(None) => {
            if args.machine_readable {
                print!("-1");
            } else {
                println!("Files are equal");
            }
            ExitCode::SUCCESS
        }
        Ok(Some(offset)) => {
            if args.machine_readable {
                print!("{}", offset);
            } else {
                println!("Files differ at byte {}", offset);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
