use clap::Parser;
use file_cmp::{
    compare_dirs, compare_files, is_dir, parse_chunk_size, FileDiff, DEFAULT_CHUNK_SIZE,
};
use std::process::ExitCode;

/// Exit code for identical files (standard cmp behavior)
const EXIT_IDENTICAL: u8 = 0;
/// Exit code for different files (standard cmp behavior)
const EXIT_DIFFERENT: u8 = 1;
/// Exit code for errors (standard cmp behavior)
const EXIT_ERROR: u8 = 2;

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
    /// Optional flag to only output non-equal results (when diffing dirs)
    #[arg(short, long)]
    diffs_only: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let chunk_size = args
        .chunk_size
        .as_ref()
        .and_then(|s| parse_chunk_size(s))
        .unwrap_or(DEFAULT_CHUNK_SIZE);

    match is_dir(&args.path1) {
        Ok(true) => match compare_dirs(&args.path1, &args.path2, args.quick, chunk_size) {
            Ok(results) => {
                let mut has_diff = false;
                let mut has_error = false;
                for (path, file_diff) in &results {
                    if args.diffs_only && *file_diff == FileDiff::Equal {
                        continue;
                    }
                    if matches!(file_diff, FileDiff::Error(_)) {
                        has_error = true;
                    }
                    if *file_diff != FileDiff::Equal {
                        has_diff = true;
                    }
                    println!(
                        "{}\t{}{}",
                        file_diff.as_number(),
                        path.display(),
                        if args.machine_readable {
                            "".to_string()
                        } else {
                            format!("\t({})", file_diff.as_desc())
                        }
                    );
                }
                if has_error {
                    ExitCode::from(EXIT_ERROR)
                } else if has_diff {
                    ExitCode::from(EXIT_DIFFERENT)
                } else {
                    ExitCode::from(EXIT_IDENTICAL)
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::from(EXIT_ERROR)
            }
        },
        Ok(false) => match compare_files(&args.path1, &args.path2, args.quick, chunk_size) {
            Ok(result) => {
                if args.machine_readable {
                    println!("{}", result.as_number())
                } else {
                    println!(
                        "{}",
                        match result {
                            FileDiff::Equal => "Files are equal".to_string(),
                            FileDiff::Different(o) => {
                                format!("Files differ at byte {}", o)
                            }
                            _ => "This should never happen.".to_string(),
                        }
                    )
                }
                match result {
                    FileDiff::Equal => ExitCode::from(EXIT_IDENTICAL),
                    _ => ExitCode::from(EXIT_DIFFERENT),
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::from(EXIT_ERROR)
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(EXIT_ERROR)
        }
    }
}
