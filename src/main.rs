use std::fs;
use std::time::Instant;

mod tag_processing;
use tag_processing::*;

fn get_file_extension(entry: &fs::DirEntry) -> Option<String> {
    return match entry.path().extension() {
        Some(extension) => Some(extension.to_string_lossy().to_lowercase()),
        _ => None,
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let entries = fs::read_dir("/Users/leopnt/Music/TCOTC").unwrap();

    for entry in entries {
        let entry = entry.as_ref().unwrap();

        match get_file_extension(entry).as_deref() {
            Some("aiff") => process_aiff(entry),
            Some("aif") => process_aiff(entry),
            Some("mp3") => process_mp3(entry),
            _ => {
                continue;
            }
        }

        println!("Name: {}", entry.path().clone().display());
    }

    let elapsed_time = start_time.elapsed();
    println!("Program took {:?}", elapsed_time);

    Ok(())
}
