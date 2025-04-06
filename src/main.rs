use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Mutex;

const BAR_TEMPLATE: &str = "{elapsed_precise:>8} | {binary_bytes_per_sec:<12} [{bar:40.red}] {bytes:>10} / {total_bytes:<10} {msg}";
const BAR_CHARS: &str = "=> ";

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let flac_files = find_flac_files(&current_dir);
    let total_files = flac_files.len() as u64;
    println!("Found {} flac files.", total_files);

    let pb = ProgressBar::new(total_files);
    pb.set_style(ProgressStyle::with_template(BAR_TEMPLATE).unwrap().progress_chars(BAR_CHARS));

    let failed_files = Mutex::new(Vec::new());

    flac_files.par_iter().for_each(|flac_file| {
        if convert_to_opus(flac_file) {
            fs::remove_file(flac_file).unwrap();
        } else {
            failed_files.lock().unwrap().push(flac_file.clone());
        }
        pb.inc(1);
    });

    pb.finish_with_message("Conversion complete");

    let failed_files = failed_files.into_inner().unwrap();
    if !failed_files.is_empty() {
        println!("Failed to convert the following files:");
        for file in failed_files {
            println!("{}", file);
        }
    }
}

fn find_flac_files(dir: &Path) -> Vec<String> {
    let mut flac_files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                flac_files.extend(find_flac_files(&path));
            } else if let Some(extension) = path.extension() {
                if extension == "flac" {
                    flac_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    flac_files
}

fn convert_to_opus(flac_file: &str) -> bool {
    let output_file = flac_file.replace(".flac", ".opus");
    let status = Command::new("opusenc")
        .arg("--vbr")
        .arg("--bitrate")
        .arg("320")
        .arg("--comment")
        .arg("comment=Encodage VBR OPUS 320 kbps by Antidote")
        .arg(flac_file)
        .arg(&output_file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Failed to execute opusenc");

    status.success()
}