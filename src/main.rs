use std::fs;
use std::path::Path;
use std::time::Instant;

mod utils;

mod track;
use track::Track;

mod track_database;
use track_database::build_track_database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let source_folder = Path::new("/Users/leopnt/Desktop/TCOTC_TEST");
    let target_folder = Path::new("/Users/leopnt/Desktop/TCOTC_TEST_TARGET");

    let source_entries = fs::read_dir(&source_folder).unwrap();
    //let target_entries = fs::read_dir("/Users/leopnt/Desktop/test-comment-music").unwrap();

    let source_tracks = build_track_database(source_entries);

    for track in source_tracks.iter() {
        track.copy_to_target_paths(target_folder);
    }

    println!("Count {}", source_tracks.len());

    let elapsed_time = start_time.elapsed();
    println!("Program took {:?}", elapsed_time);

    Ok(())
}
