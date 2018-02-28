extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tree_magic;
extern crate walkdir;
extern crate xdg;

mod config;
mod import_set;

use std::fs::{copy, create_dir_all, remove_file, File};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use chrono::Duration;
use chrono::prelude::*;
use walkdir::WalkDir;

use import_set::ImportSet;

pub struct MediaImport;

impl MediaImport {
    pub fn new() -> Self {
        MediaImport {}
    }

    fn get_media_creation_date(&self, path: &PathBuf) -> Result<chrono::NaiveDate, failure::Error> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let time_offset = NaiveDateTime::from_timestamp(0, 0);

        let time = metadata.created().or(metadata.modified())?;
        let std_duration = time.duration_since(UNIX_EPOCH)?;
        let duration = Duration::from_std(std_duration)?;
        let t = time_offset + duration;
        Ok(t.date())
    }

    fn move_file(
        &self,
        target_dir: &PathBuf,
        year: i32,
        dir: &String,
        media_subdir: &str,
        file: &PathBuf,
    ) -> Result<(), failure::Error> {
        let year_path = target_dir.join(year.to_string());
        let dir_path = target_dir.join(year_path.join(dir));
        let media_path = dir_path.join(media_subdir);
        create_dir_all(&media_path)?;
        let file_name = file.file_name()
            .expect("expect input path to have a file name");
        let target_path = media_path.join(file_name);
        // rename does not work here, hence we're copying and deleting manually
        copy(file, target_path)?;
        remove_file(file)?;
        Ok(())
    }

    pub fn import(&self, path: &PathBuf) -> Result<(), failure::Error> {
        let config = config::get_user_config()?;
        let mut imports = ImportSet::new();
        let mut other = 0;

        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            let mime_type = tree_magic::from_filepath(entry.path());
            let media_mime: mime::Mime =
                mime_type.parse().expect("could not parse media mime type");
            if !imports.add_media(entry.path().to_path_buf(), media_mime) {
                other += 1;
            }
        }

        println!(
            "found {} images, {} videos and {} other files.",
            imports.images().len(),
            imports.videos().len(),
            other,
        );

        for img in imports.images() {
            let d = self.get_media_creation_date(img)?;
            self.move_file(
                config.target_path(),
                d.year(),
                &format!("{:02}{:02}", d.month(), d.day()),
                "Images",
                &img,
            )?;
        }
        for vid in imports.videos() {
            let d = self.get_media_creation_date(vid)?;
            self.move_file(
                config.target_path(),
                d.year(),
                &format!("{:02}{:02}", d.month(), d.day()),
                "Videos",
                &vid,
            )?;
        }

        Ok(())
    }
}
