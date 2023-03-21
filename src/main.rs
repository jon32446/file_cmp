use clap::Parser;
use file_cmp::{compare_dirs, compare_files, is_dir, FileDiff};
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

            for (path, file_diff) in results {
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
            ExitCode::SUCCESS
        }
        Ok(false) => match compare_files(&args.path1, &args.path2, args.quick) {
            Ok(result @ _) => {
                if args.machine_readable {
                    print!("{}", result.as_number())
                } else {
                    print!(
                        "{}",
                        match result {
                            FileDiff::Equal => "Files are equal".to_string(),
                            FileDiff::Different(o @ _) => {
                                format!("Files differ at byte {}", o)
                            }
                            _ => "This should never happen.".to_string(),
                        }
                    )
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
