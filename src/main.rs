extern crate glob;
extern crate rayon;
extern crate indicatif;

use glob::glob;
use std::{error::Error, process::{Command, Stdio}, env};
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

fn main() -> Result<(), Box<dyn Error>> {
    // Set the root directory
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("expected one argument: the root directory".into());
    }
    let root_dir = &args[1];

    // Find all wav files in the root directory and its subdirectories
    let flac_paths: Vec<_> = glob(&format!("{}/**/*.flac", root_dir))?
        .filter_map(Result::ok)
        .collect();

    println!("Found {} flac files", flac_paths.len());

    // filter out the flac files that already have an opus file
    let flac_paths: Vec<_> = flac_paths
        .iter()
        .filter(|flac_path| {
            let opus_path = flac_path.with_extension("opus");
            !opus_path.exists()
        })
        .collect();

    // Process the wav files in parallel
    flac_paths.par_iter().progress_count(flac_paths.len() as u64).for_each(|flac_path| {
        // Get the path to the opus file
        let opus_path = flac_path.with_extension("opus");

        // Use ffmpeg to convert the flac file to opus
        let status = Command::new("opusenc")
            .arg("--vbr")
            .arg("--bitrate")
            .arg("320")
            .arg("--comment")
            .arg("comment=Encodage VBR OPUS 320 kbps by Antidote")
            .arg(flac_path)
            .arg(opus_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();

        if !status.success() {
            panic!("ffmpeg failed with status {}", status);
        }
    });

    Ok(())
}