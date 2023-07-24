use crate::track::Track;
use id3::Tag;
use std::error::Error;
use std::fs::DirEntry;

pub fn process_aiff(entry: &DirEntry) -> Result<Track, Box<dyn Error>> {
    let tag = Tag::read_from_aiff_path(entry.path()).unwrap();
    Track::from(&tag, "aiff".to_string(), entry.path())
}

pub fn process_mp3(entry: &DirEntry) -> Result<Track, Box<dyn Error>> {
    let tag = Tag::read_from_path(entry.path()).unwrap();
    Track::from(&tag, "aiff".to_string(), entry.path())
}
