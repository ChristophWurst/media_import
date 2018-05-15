use std::path::PathBuf;

use mime::{self, Mime};

pub struct ImportSet {
    audio: Vec<PathBuf>,
    images: Vec<PathBuf>,
    videos: Vec<PathBuf>,
}

impl ImportSet {
    pub fn new() -> Self {
        ImportSet {
            audio: vec![],
            images: vec![],
            videos: vec![],
        }
    }

    pub fn add_media(&mut self, path: PathBuf, media_mime: Mime) -> bool {
        if !path.is_file() {
            return false;
        }

        match (
            media_mime.type_(),
            path.extension().and_then(|osstr| osstr.to_str()),
        ) {
            (mime::AUDIO, _) => {
                self.audio.push(path);
                true
            }
            (_, Some("MP3")) => {
                self.audio.push(path);
                true
            }
            (_, Some("mp3")) => {
                self.audio.push(path);
                true
            }
            (_, Some("WAV")) => {
                self.audio.push(path);
                true
            }
            (_, Some("wav")) => {
                self.audio.push(path);
                true
            }
            (mime::IMAGE, _) => {
                self.images.push(path);
                true
            }
            (mime::VIDEO, _) => {
                self.videos.push(path);
                true
            }
            _ => false,
        }
    }

    pub fn audio(&self) -> &Vec<PathBuf> {
        &self.audio
    }

    pub fn images(&self) -> &Vec<PathBuf> {
        &self.images
    }

    pub fn videos(&self) -> &Vec<PathBuf> {
        &self.videos
    }
}
