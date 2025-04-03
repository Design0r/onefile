# OneFile: Directory File Concatenator with Clipboard Integration

OneFile is a Rust command-line application that recursively reads all valid UTF-8 files from specified directories, concatenates their contents with clear visual separators, and copies the aggregated text to your system clipboard. Files that are not valid UTF-8 are skipped to ensure smooth execution.

---

## Features

- **Recursive Directory Traversal:**  
  Walk through a directory tree to process every file.

- **UTF-8 Validation:**  
  Reads files as raw bytes and attempts UTF-8 conversion; skips files that are not valid UTF-8.

- **Content Aggregation:**  
  Each file's content is prefixed with a separator and the file path, then all valid contents are combined into one string.

- **Clipboard Integration:**  
  Automatically copies the concatenated content to the system clipboard using the `clipboard` crate.

- **Command-Line Interface:**  
  Leverages `clap` for easy and intuitive command-line argument parsing.


## Installation


   ```bash
    git clone https://github.com/Design0r/onefile.git
    cd onefile
    cargo install --path .
   ```

## Usage

   ```bash
    onefile --help

    Usage: onefile [INPUTS]...

    Arguments:
      [INPUTS]...

    Options:
      -h, --help     Print help
      -V, --version  Print version
   ```

## Example

   ```bash
    onefile ./my_project ../another_project ./just_another_folder
   ```
