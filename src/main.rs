use clap::Parser;
use file_cmp::{compare_dirs, compare_files, is_dir};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// Path to first file or directory to compare
    path1: String,
    /// Path to second file or directory to compare
    path2: String,
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

    match is_dir(&args.path1) {
        Ok(true) => {
            let results = compare_dirs(&args.path1, &args.path2, args.quick);

            for (path, offset) in results {
                let status = match offset {
                    -1 => "equal",
                    -2 => "left only",
                    -3 => "right only",
                    _ => "diff",
                };
                println!("{}\t{}\t({})", offset, path.display(), status);
            }
            ExitCode::SUCCESS
        }
        Ok(false) => match compare_files(&args.path1, &args.path2, args.quick) {
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
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
