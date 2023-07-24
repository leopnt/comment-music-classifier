use std::fs;
use std::path::{Path, PathBuf};
use std::str::Chars;
use std::time::Instant;

mod tag_processing;
use tag_processing::*;

mod track;
use track::Track;

fn get_file_extension(entry: &fs::DirEntry) -> Option<String> {
    return match entry.path().extension() {
        Some(extension) => Some(extension.to_string_lossy().to_lowercase()),
        _ => None,
    };
}

fn build_track_db(entries: fs::ReadDir) -> Vec<Track> {
    let mut tracks = Vec::<Track>::new();

    for entry in entries {
        let entry = entry.as_ref().unwrap();
        match get_file_extension(entry).as_deref() {
            Some("aiff") | Some("aif") => {
                if let Ok(track) = process_aiff(entry) {
                    tracks.push(track);
                }
            }
            Some("mp3") => {
                if let Ok(track) = process_mp3(entry) {
                    tracks.push(track);
                }
            }
            _ => {
                continue;
            }
        }
    }

    tracks
}

fn build_custom_track_name(track: &Track) -> String {
    format!(
        "{} {} - {}.{}",
        track.custom_comment, track.title, track.artist, track.file_extension
    )
}

// get classifiers from track's custom comment. E.g. "2abc" -> "a", "b", "c"
fn get_track_classifiers(track: &Track) -> Chars {
    let classifiers = &track.custom_comment[1..];

    classifiers.chars()
}

// "2abc"
// will give:
// ["<source folder>/a/<custom track name>",
// "<source folder>/b/<custom track name>",
// "<source folder>/c/<custom track name>"]
fn build_custom_target_paths(track: &Track, target_folder: &Path) -> Vec<PathBuf> {
    get_track_classifiers(track)
        .map(|classifier| {
            target_folder
                .join(classifier.to_string())
                .join(build_custom_track_name(track))
        })
        .collect()
}

fn copy_file_with_warning(source_path: &Path, dest_path: &PathBuf) -> std::io::Result<()> {
    let mut dest_path = dest_path.clone();
    let dest_path_display = &dest_path.display().to_string();

    if dest_path_display.len() > 255 {
        println!(
            "Warning: The filename {} is too long and will be truncated.",
            dest_path.display()
        );
        let ext = dest_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let truncated_filename = dest_path_display
            .chars()
            .take(255 - 1 - ext.len())
            .collect::<String>();
        dest_path = PathBuf::from(truncated_filename + "." + ext);
    }

    if dest_path.exists() {
        println!(
            "Warning: The file {} already exists in the destination.",
            dest_path.display()
        );
        return Ok(());
    }

    // create directory if does not exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source_path, dest_path)?;
    Ok(())
}

fn copy_track_to_target_paths(track: &Track, target_folder: &Path) {
    for target_path in build_custom_target_paths(track, target_folder) {
        // attempt to copy the file to the destination folder
        match copy_file_with_warning(&track.source_path, &target_path) {
            Ok(_) => println!("Copied: {:?}", target_path),
            Err(e) => eprintln!("Failed to copy: {:?}. Error: {}", target_path, e),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let source_folder = Path::new("/Users/leopnt/Desktop/TCOTC_TEST");
    let target_folder = Path::new("/Users/leopnt/Desktop/TCOTC_TEST_TARGET");

    let source_entries = fs::read_dir(&source_folder).unwrap();
    //let target_entries = fs::read_dir("/Users/leopnt/Desktop/test-comment-music").unwrap();

    let source_tracks = build_track_db(source_entries);

    for track in source_tracks.iter() {
        copy_track_to_target_paths(track, target_folder);
    }

    println!("Count {}", source_tracks.len());

    let elapsed_time = start_time.elapsed();
    println!("Program took {:?}", elapsed_time);

    Ok(())
}
