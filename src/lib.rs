extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate indicatif;
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
use std::thread;
use std::time::UNIX_EPOCH;

use config::UserConfig;

use chrono::Duration;
use chrono::prelude::*;
use indicatif::{MultiProgress, ProgressBar};
use walkdir::WalkDir;

use import_set::ImportSet;

pub struct MediaImport;

fn get_media_creation_date(path: &PathBuf) -> Result<chrono::NaiveDate, failure::Error> {
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

fn move_files(
    imports: ImportSet,
    config: UserConfig,
    progress: ProgressBar,
) -> Result<(), failure::Error> {
    for audio in imports.audio() {
        progress.inc(1);
        let d = get_media_creation_date(audio)?;
        move_file(
            config.target_path(),
            d.year(),
            &format!("{:02}{:02}", d.month(), d.day()),
            "Audio",
            &audio,
        )?;
    }
    for img in imports.images() {
        progress.inc(1);
        let d = get_media_creation_date(img)?;
        move_file(
            config.target_path(),
            d.year(),
            &format!("{:02}{:02}", d.month(), d.day()),
            "Images",
            &img,
        )?;
    }
    for vid in imports.videos() {
        progress.inc(1);
        let d = get_media_creation_date(vid)?;
        move_file(
            config.target_path(),
            d.year(),
            &format!("{:02}{:02}", d.month(), d.day()),
            "Videos",
            &vid,
        )?;
    }
    progress.finish();
    Ok(())
}

impl MediaImport {
    pub fn new() -> Self {
        MediaImport {}
    }

    pub fn import<P>(&self, path: P) -> Result<(), failure::Error>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let mp = MultiProgress::new();
        let prog_scan = mp.add(ProgressBar::new_spinner());
        prog_scan.set_message("Scanning directory...");

        let config = config::get_user_config()?;

        let scan_t = thread::spawn(move || {
            let mut imports = ImportSet::new();
            let mut other = 0;

            for entry in WalkDir::new(path) {
                prog_scan.tick();

                let entry = entry.unwrap();
                let mime_type = tree_magic::from_filepath(entry.path());
                let media_mime: mime::Mime =
                    mime_type.parse().expect("could not parse media mime type");
                if !imports.add_media(entry.path().to_path_buf(), media_mime) {
                    other += 1;
                } else {
                    prog_scan.set_message(&format!(
                        "found {} audio, {} images, {} videos and {} other files.",
                        imports.audio().len(),
                        imports.images().len(),
                        imports.videos().len(),
                        other,
                    ));
                }
            }

            prog_scan.finish_with_message(&format!(
                "found {} audio, {} images, {} videos and {} other files.",
                imports.audio().len(),
                imports.images().len(),
                imports.videos().len(),
                other,
            ));

            imports
        });

        mp.join()?;
        let imports = scan_t.join().unwrap();


        let num_files = imports.audio().len() + imports.images().len() + imports.videos().len();
        let prog_move = mp.add(ProgressBar::new(num_files as u64));

        let move_t = thread::spawn(move || move_files(imports, config, prog_move));

        mp.join()?;
        move_t.join().unwrap()?;

        Ok(())
    }
}
