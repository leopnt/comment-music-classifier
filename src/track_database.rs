use std::fs;

use crate::Track;

pub fn build_track_database(entries: fs::ReadDir) -> Vec<Track> {
    let mut tracks = Vec::<Track>::new();

    for entry in entries {
        let entry = entry.unwrap();

        if entry.path().is_dir() {
            // If the entry is a directory, recursively call the function
            let sub_entries = fs::read_dir(entry.path()).expect("Failed to read directory");
            let sub_tracks = build_track_database(sub_entries);
            tracks.extend(sub_tracks);
        } else if let Ok(track) = Track::from_pathbuf(entry.path()) {
            tracks.push(track);
        } else if let Err(e) = Track::from_pathbuf(entry.path()) {
            println!(
                "Track parsing failed for: {:?} Error is: {:?}",
                entry.path(),
                e
            );
        }
    }

    tracks
}

// returns db_left - db_right
pub fn substract_track_databases<'a>(
    db_left: &'a Vec<Track>,
    db_right: &Vec<Track>,
) -> Vec<&'a Track> {
    let mut difference = Vec::<&Track>::new();
    for track in db_left {
        if !db_right.contains(&track) {
            difference.push(&track);
        }
    }

    difference
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_substract_track_databases() {
        let track_a = Track {
            title: "title_a".to_string(),
            artist: "artist_a".to_string(),
            custom_comment: "3cba".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/a"),
        };

        let track_b = Track {
            title: "title_b".to_string(),
            artist: "artist_b".to_string(),
            custom_comment: "2abc".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/b"),
        };

        let track_b_bis = Track {
            title: "title_b".to_string(),
            artist: "artist_b".to_string(),
            custom_comment: "2abc".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/b/bis"),
        };

        let track_c = Track {
            title: "title_c".to_string(),
            artist: "artist_c".to_string(),
            custom_comment: "1d".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/c"),
        };

        let db_left = vec![track_a.clone(), track_b];
        let db_right = vec![track_b_bis, track_c];

        // PartialEq for Track makes track_b and track_b_bis the same
        // (same title and artist)

        assert_eq!(1, substract_track_databases(&db_left, &db_right).len());

        assert_eq!(track_a, *substract_track_databases(&db_left, &db_right)[0]);
    }
}
