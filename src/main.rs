use anyhow::Result;
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{fs, path::Path};
use walkdir::WalkDir;

mod cli;

const SEPARATOR: &str = "========================================\n";

fn read_file(path: &Path) -> Result<Option<String>> {
    // Read the file as bytes
    let bytes = fs::read(path.canonicalize()?.as_os_str())?;

    // Try converting to a &str
    match std::str::from_utf8(&bytes) {
        Ok(text) => {
            let combined = format!(
                "{}{:?}\n{}\n{}",
                SEPARATOR,
                path.as_os_str(),
                SEPARATOR,
                text
            );
            Ok(Some(combined))
        }
        Err(_) => {
            // If invalid UTF-8, skip the file by returning None
            Ok(None)
        }
    }
}

fn walk_dir(path: &Path) -> Result<Vec<String>> {
    let mut results = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            if let Ok(Some(content)) = read_file(entry.path()) {
                results.push(content);
            }
        }
    }
    Ok(results)
}

fn main() -> Result<()> {
    let args = cli::Args::parse();

    for path in &args.get_absolute_paths() {
        println!("{:?} {}", path, path.is_file());
    }

    let mut content = Vec::new();
    for path in &args.inputs {
        match walk_dir(path) {
            Ok(files) => content.extend(files),
            Err(e) => println!("Error processing directory {:?}: {}", path, e),
        }
    }

    for s in &content {
        println!("{}", s);
    }

    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to create clipboard context");

    ctx.set_contents(content.join("\n"))
        .expect("Failed to set clipboard contents");

    println!("{:?} Files detected", content.len());
    println!("Text has been copied to the clipboard!");

    Ok(())
}
