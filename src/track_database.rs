use std::fs;

use crate::Track;

pub fn build_track_database(entries: fs::ReadDir) -> Vec<Track> {
    let mut tracks = Vec::<Track>::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            // If the entry is a directory, recursively call the function
            let sub_entries = fs::read_dir(path).expect("Failed to read directory");
            let sub_tracks = build_track_database(sub_entries);
            tracks.extend(sub_tracks);
        } else if let Ok(track) = Track::from_pathbuf(entry.path()) {
            tracks.push(track);
        } else if let Err(e) = Track::from_pathbuf(entry.path()) {
            println!("{:?}", e);
        }
    }

    tracks
}
