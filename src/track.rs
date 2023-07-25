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
pub struct CustomCommentEmptyError;

impl Error for CustomCommentEmptyError {}

#[derive(Debug)]
pub struct FileExtensionNotSupportedError {
    extension: String,
}

impl fmt::Display for FileExtensionNotSupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File extension not supported: {}", self.extension)
    }
}

impl Error for FileExtensionNotSupportedError {}

impl fmt::Display for CustomCommentEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty error for comment")
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
    title: String,
    artist: String,
    custom_comment: String,
    file_extension: String,
    source_path: PathBuf,
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
            let mut parsed_comment = parse_custom_comment(&comment.text)?;
            parsed_comment.sort();

            custom_comment = parsed_comment.join("");
        }

        if custom_comment.is_empty() {
            return Err(Box::new(CustomCommentEmptyError));
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
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        let same_title = self.title.to_lowercase() == other.title.to_lowercase();
        let same_artist = self.artist.to_lowercase() == other.artist.to_lowercase();

        same_title && same_artist
    }
}

#[cfg(test)]
mod test {
    use super::sanitize_filename;

    #[test]
    fn test_sanitize_path() {
        assert_eq!(
            "2af Title - Super Megà _ Artist.aiff",
            sanitize_filename("2af Title - Super Megà / Artist.aiff")
        );
    }
}
