use id3::{Tag, TagLike};
use regex::Regex;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CustomCommentParseError {
    comment: String,
}

impl fmt::Display for CustomCommentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing error for comment: {}", self.comment)
    }
}

impl Error for CustomCommentParseError {}

#[derive(Debug)]
pub struct CustomCommentEmptyError;

impl fmt::Display for CustomCommentEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty error for comment")
    }
}

impl Error for CustomCommentEmptyError {}

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

#[derive(Debug, Clone)]
pub struct Track {
    title: String,
    artist: String,
    custom_comment: String,
    src_path: String,
}

impl Track {
    pub fn from(tag: &Tag) -> Result<Self, Box<dyn Error>> {
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

        let src_path = "".to_string();

        Ok(Track {
            title,
            artist,
            custom_comment,
            src_path,
        })
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        let same_title = self.title.to_lowercase() == other.title.to_lowercase();
        let same_artist = self.artist.to_lowercase() == other.artist.to_lowercase();

        same_title && same_artist
    }
}
