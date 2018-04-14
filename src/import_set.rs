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
        if media_mime.type_() == mime::AUDIO {
            self.audio.push(path);
            true
        } else if media_mime.type_() == mime::IMAGE {
            self.images.push(path);
            true
        } else if media_mime.type_() == mime::VIDEO {
            self.videos.push(path);
            true
        } else {
            false
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
