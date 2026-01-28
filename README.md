# file_cmp

A high-performance file comparison utility written in Rust.

## Features

- **Fast**: Uses parallel processing (rayon) for directory comparisons
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Flexible**: Configurable chunk size, quick mode, and machine-readable output
- **Reliable**: Proper error handling with meaningful exit codes

## Usage

The utility can compare files or directories.

```
Usage: file_cmp [OPTIONS] <PATH1> <PATH2>

Arguments:
  <PATH1>  Path to first file or directory to compare
  <PATH2>  Path to second file or directory to compare

Options:
  -m, --machine                  Optional flag to enable machine-readable output
  -q, --quick                    Optional flag to do faster comparison and not output first diff offset
  -c, --chunk-size <CHUNK_SIZE>  Optional parameter to set the chunk size for reading the files, e.g. 4k, 2M
  -d, --diffs-only               Optional flag to only output non-equal results (when diffing dirs)
  -h, --help                     Print help
  -V, --version                  Print version
```

## Exit Codes

Following standard `cmp` behavior:
- **0**: Files are identical
- **1**: Files differ
- **2**: Error occurred

### Examples:

```
> file_cmp /path/to/dir1 /path/to/dir2
25      /path/to/dir1/file.lib       (diff)
-1      /path/to/dir1/equal_file.txt (equal)
-2      /path/to/dir1/leftonly.txt   (left only)
-3      /path/to/dir2/rightonly.lib  (right only)

> file_cmp file1.bin file2.bin
Files differ at byte 25

> file_cmp -m file1.bin file2.bin
25
```

## Performance

Benchmark results comparing file_cmp against the standard `cmp` utility:

| Test Case | cmp | file_cmp | file_cmp -q |
|-----------|-----|----------|-------------|
| 1MB identical | 4ms | 2ms | 1ms |
| 10MB identical | 4ms | 4ms | 4ms |
| 100MB identical | 33ms | 27ms | 24ms |
| 10MB differ at start | 1ms | 1ms | 1ms |
| 10MB differ at end | 4ms | 4ms | 4ms |

*Results may vary based on hardware and system load.*

Run the included benchmark: `./benchmark.sh`

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```
