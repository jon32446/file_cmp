# file_cmp

A file comparison utility written in Rust.

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
  -h, --help                     Print help
  -V, --version                  Print version
```

### Examples:

```
> file_cmp C:\Python26\libs C:\Python27\libs
25      C:\Python26\libs\bz2.lib        (diff)
-1      C:\Python26\libs\equal_file.txt (equal)
-2      C:\Python26\libs\leftonly.txt   (left only)
-3      C:\Python27\libs\python27.lib   (right only)

> file_cmp C:\Python26\libs\bz2.lib C:\Python27\libs\bz2.lib
Files differ at byte 25

> file_cmp -m C:\Python26\libs\bz2.lib C:\Python27\libs\bz2.lib
25

```
