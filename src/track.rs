use id3::{Tag, TagLike};
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::Chars;

use crate::utils::{self, get_file_extension};

#[derive(Debug)]
pub struct CustomCommentParseError {
    comment: String,
}

impl Error for CustomCommentParseError {}

impl fmt::Display for CustomCommentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing error for comment: {}", self.comment)
    }
}

#[derive(Debug)]
pub struct ValidCommentNotFoundError;

impl Error for ValidCommentNotFoundError {}

impl fmt::Display for ValidCommentNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty error for comment")
    }
}

#[derive(Debug)]
pub struct FileExtensionNotSupportedError {
    extension: String,
}

impl Error for FileExtensionNotSupportedError {}

impl fmt::Display for FileExtensionNotSupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File extension not supported: {}", self.extension)
    }
}

fn parse_custom_comment(comment: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let regex_pattern = r"^\d,[a-z;]+$";
    let regex = Regex::new(regex_pattern).unwrap();

    // Check if the input matches the regex pattern
    if !regex.is_match(comment) {
        return Err(Box::new(CustomCommentParseError {
            comment: comment.to_string(),
        }));
    }

    // Split the input string by ',' and ';' and collect the result into a Vec<String>
    let result: Vec<String> = comment
        .split(|c| c == ',' || c == ';')
        .map(|s| s.to_string())
        .collect();

    Ok(result)
}

fn is_illegal_filename_char(c: char) -> bool {
    c == '/' || c == '<' || c == '>' || c == ':' || c == '\\' || c == '|' || c == '?' || c == '*'
}

fn sanitize_filename(filename: &str) -> String {
    filename.replace(|c: char| is_illegal_filename_char(c), "_")
}

#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub custom_comment: String,
    pub file_extension: String,
    pub source_path: PathBuf,
}

impl Track {
    pub fn from_pathbuf(source_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let file_extension =
            get_file_extension(&source_path).ok_or(Box::new(FileExtensionNotSupportedError {
                extension: "".to_string(),
            }))?;

        let tag = match file_extension.as_str() {
            "aiff" | "aif" => Tag::read_from_aiff_path(&source_path),
            "mp3" => Tag::read_from_path(&source_path),
            extension => {
                return Err(Box::new(FileExtensionNotSupportedError {
                    extension: extension.to_string(),
                }))
            }
        }?;

        let title = tag.title().ok_or("No title ID3 tag found")?.to_string();
        let artist = tag.artist().ok_or("No artist ID3 tag found")?.to_string();

        let mut custom_comment = "".to_string();
        for comment in tag.comments() {
            if let Ok(parsed_comment) = parse_custom_comment(&comment.text).as_mut() {
                parsed_comment.sort();
                custom_comment = parsed_comment.join("");
                break;
            }
        }

        if custom_comment.is_empty() {
            return Err(Box::new(ValidCommentNotFoundError));
        }

        Ok(Track {
            title,
            artist,
            custom_comment,
            file_extension,
            source_path,
        })
    }

    pub fn build_custom_filename(&self) -> String {
        sanitize_filename(
            &format!(
                "{} {} - {}.{}",
                self.custom_comment, self.title, self.artist, self.file_extension
            )
            .to_string(),
        )
    }

    // get classifiers from track's custom comment. E.g. "2abc" -> "a", "b", "c"
    fn get_classifiers(&self) -> Chars {
        let classifiers = &self.custom_comment[1..];

        classifiers.chars()
    }

    // "2abc"
    // will give:
    // ["<source folder>/a/<custom track name>",
    // "<source folder>/b/<custom track name>",
    // "<source folder>/c/<custom track name>"]
    fn build_custom_target_paths(&self, target_folder: &Path) -> Vec<PathBuf> {
        self.get_classifiers()
            .map(|classifier| {
                target_folder
                    .join(classifier.to_string())
                    .join(self.build_custom_filename())
            })
            .collect()
    }

    pub fn copy_to_target_paths(&self, target_folder: &Path) {
        for target_path in self.build_custom_target_paths(target_folder) {
            // attempt to copy the file to the destination folder
            match utils::copy_file_with_warning(&self.source_path, &target_path) {
                Ok(_) => println!("Copied: {:?}", target_path),
                Err(e) => eprintln!("Failed to copy: {:?}. Error: {}", target_path, e),
            }
        }
    }

    pub fn move_to_folder(&self, destination_folder: &PathBuf) {
        let source_file = self.source_path.clone();

        // Extract the source filename from the path
        let source_filename = Path::new(&source_file)
            .file_name()
            .expect("Invalid source file path")
            .to_str()
            .expect("Invalid source file name")
            .to_string();

        let destination_file = PathBuf::from(destination_folder).join(source_filename);

        match std::fs::rename(&source_file, &destination_file) {
            Ok(_) => println!("Moved: {:?} to {:?}", source_file, destination_folder),
            Err(e) => eprintln!("Failed to move: {:?}. Error: {}", destination_file, e),
        }
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.build_custom_filename() == other.build_custom_filename()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sanitize_path() {
        assert_eq!(
            "2af Title - Super Megà _ Artist.aiff",
            sanitize_filename("2af Title - Super Megà / Artist.aiff")
        );
    }

    #[test]
    fn test_track_partialeq() {
        let track_a = Track {
            title: "title_a".to_string(),
            artist: "artist_a".to_string(),
            custom_comment: "3abc".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/a"),
        };

        let track_a_same = Track {
            title: "title_a".to_string(),
            artist: "artist_a".to_string(),
            custom_comment: "3abc".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/c/a"),
        };

        let track_c = Track {
            title: "title_c".to_string(),
            artist: "artist_a".to_string(),
            custom_comment: "3abc".to_string(),
            file_extension: "aiff".to_string(),
            source_path: std::path::PathBuf::from("path/to/c"),
        };

        assert_eq!(track_a, track_a_same);

        assert_ne!(track_a, track_c);
    }
}
