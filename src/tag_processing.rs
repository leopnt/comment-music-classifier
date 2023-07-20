use id3::{Tag, TagLike};
use std::fs::DirEntry;

pub fn process_tag(tag: &Tag) {
    if let Some(artist) = tag.artist() {
        println!("artist: {}", artist);
    }
    if let Some(title) = tag.title() {
        println!("title: {}", title);
    }

    for comment in tag.comments() {
        println!("comment: {}", comment.text);
    }
}

pub fn process_aiff(entry: &DirEntry) {
    let tag = Tag::read_from_aiff_path(entry.path()).unwrap();
    process_tag(&tag);
}

pub fn process_mp3(entry: &DirEntry) {
    let tag = Tag::read_from_path(entry.path()).unwrap();
    process_tag(&tag);
}
