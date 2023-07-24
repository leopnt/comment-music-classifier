use std::fs;
use std::path::PathBuf;

pub fn get_file_extension(path: &PathBuf) -> Option<String> {
    return match path.extension() {
        Some(e) => Some(e.to_string_lossy().to_lowercase()),
        _ => None,
    };
}

pub fn copy_file_with_warning(source_path: &PathBuf, dest_path: &PathBuf) -> std::io::Result<()> {
    let mut dest_path = dest_path.clone();
    let dest_path_display = &dest_path.display().to_string();

    if dest_path_display.len() > 255 {
        println!(
            "Warning: The filename {} is too long and will be truncated.",
            dest_path.display()
        );

        let ext = get_file_extension(&dest_path).unwrap();
        let truncated_filename = dest_path_display
            .chars()
            .take(255 - 1 - ext.len())
            .collect::<String>();
        dest_path = PathBuf::from(truncated_filename + "." + &ext);
    }

    if dest_path.exists() {
        println!(
            "Warning: The file {} already exists in the destination.",
            dest_path.display()
        );
        return Ok(());
    }

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source_path, dest_path)?;
    Ok(())
}
