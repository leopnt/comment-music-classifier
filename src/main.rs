use std::fs;
use std::path::Path;
use std::time::Instant;

use clap::Parser;

mod utils;

mod track;
use track::Track;

mod track_database;
use track_database::build_track_database;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to analyse and copy the tracks from
    #[arg(short, long)]
    source: String,

    /// The target directory to copy the classified tracks into
    #[arg(short, long)]
    target: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let args = Args::parse();
    let source_folder = Path::new(&args.source);
    let target_folder = Path::new(&args.target);

    let source_entries = fs::read_dir(source_folder).unwrap();

    let source_tracks = build_track_database(source_entries);

    println!("Found {} valid source tracks", source_tracks.len());

    let mut counter_copied = 0;
    for track in source_tracks.iter() {
        track.copy_to_target_paths(target_folder);

        counter_copied += 1;
        let elapsed_time = start_time.elapsed();
        println!(
            "{}/{} copied. {:?}s elapsed",
            counter_copied,
            source_tracks.len(),
            elapsed_time
        );
    }

    let elapsed_time = start_time.elapsed();
    println!("Program took {:?}", elapsed_time);

    Ok(())
}
