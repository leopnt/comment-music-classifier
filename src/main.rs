use std::fs;
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
    format!("{} {} - {}.{}", track.custom_comment, track.title, track.artist, track.file_extension)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let source_entries = fs::read_dir("/Users/leopnt/Desktop/TCOTC_TEST").unwrap();
    //let target_entries = fs::read_dir("/Users/leopnt/Desktop/test-comment-music").unwrap();

    let source_tracks = build_track_db(source_entries);

    for track in source_tracks.iter() {
        println!("{}", build_custom_track_name(track));
    }

    println!("Count {}", source_tracks.len());

    let elapsed_time = start_time.elapsed();
    println!("Program took {:?}", elapsed_time);

    Ok(())
}
