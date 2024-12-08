use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the root folder path from command-line argument or use a default
    let args: Vec<String> = env::args().collect();
    let root_folder = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    // Ensure the root folder exists
    if !root_folder.is_dir() {
        return Err(format!("Path does not exist or is not a directory: {}", root_folder.display()).into());
    }

    // Iterate through each subdirectory
    for entry in fs::read_dir(&root_folder)? {
        let entry = entry?;
        let path = entry.path();
       
        // Only process directories
        if path.is_dir() {
            rename_files_in_directory(&path)?;
        }
    }
    Ok(())
}

fn rename_files_in_directory(dir_path: &Path) -> Result<(), std::io::Error> {
    let files: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if files.is_empty() {
        println!("No files found in directory: {}", dir_path.display());
        return Ok(());
    }


    let extensions: HashSet<_> = files.iter().filter_map(|file_path| file_path.extension()).collect();

    if extensions.len() > 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput, 
            format!("Multiple file extensions found")));
    }

    let mut sorted_files = files.clone();
    sorted_files.sort_by_key(|file_path| {
        file_path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
    });

    let total_files = sorted_files.len();
    let padding_width = match total_files {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        _ => 5, // For 10000 and above
    };

    for (index, file_path) in sorted_files.iter().enumerate() {
        let filename = file_path.file_name().unwrap_or_default();
        let extension = file_path.extension().unwrap_or_default();

        let new_filename = format!(
            "{:0width$}.{}",
            index + 1, 
            extension.to_str().unwrap_or(""),
            width = padding_width
        );
        
        let new_file_path = file_path.with_file_name(new_filename);

        fs::rename(file_path, &new_file_path)?;
        println!("Renamed {} to {}",
            filename.to_str().unwrap_or(""),
            new_file_path.file_name().unwrap_or_default().to_str().unwrap_or("")
        );

    }

    Ok(())
}