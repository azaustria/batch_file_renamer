# Batch File Renamer

## Overview

Rust command-line utility to systematically rename files within directories.
- Rename files alphabetically
- Apply zero-padded numbering
- Ensure consistency across file types in a directory

## Features

- Recursively processes subdirectories
- Sorts files alphabetically before renaming
- Applies zero-padded numbering based on alphabetical order

## Requirements

- Rust (latest stable version)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/batch_file_renamer.git
   cd batch_file_renamer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Usage
Run the program in the current directory:
```bash
cargo run
```

### Specify a Directory
Provide a specific directory path:
```bash
cargo run -- /path/to/target/directory
```

## Example

Suppose you have a directory with files:
```
photos/
├── sunset_b.jpg
├── sunset_a.jpg
├── sunset_c.jpg
```

After running the utility, the directory becomes:
```
photos/
├── 001.jpg
├── 002.jpg
├── 003.jpg
```

## Validation Rules

- All files in a directory must have the same file extension
- If multiple extensions are detected, the renaming operation will be cancelled

## Why? .. and Closing Notes

- For standardizing file names of files under a folder/directory because I like my files clean and organized
- Practising Rust
- I might improve or add on the features if my use case requires

## License

Distributed under the MIT License. See `LICENSE` for more information
