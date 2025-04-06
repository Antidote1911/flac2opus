use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let flac_files = find_flac_files(&current_dir);

    flac_files.par_iter().for_each(|flac_file| {
        convert_to_opus(flac_file);
    });
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

fn convert_to_opus(flac_file: &str) {
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

    if status.success() {
        println!("Successfully converted {}", flac_file);
        fs::remove_file(flac_file).unwrap();
    } else {
        eprintln!("Failed to convert {}", flac_file);
    }
}