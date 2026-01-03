use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use log::{info, warn};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    info!("starting rgx");
    info!("cwd = {:?}", std::env::current_dir()?);
    info!("searching for '{}' in {}", args.pattern, args.path.display());

    let path: PathBuf = if args.path.is_absolute() {
        args.path.clone()
    } else {
        std::env::current_dir()?.join(&args.path)
    };

    let path = path
        .canonicalize()
        .with_context(|| format!("invalid path `{}`", path.display()))?;

    let file = File::open(&path)
        .with_context(|| format!("could not open file `{}`", path.display()))?;

    let reader = BufReader::new(file);

    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("searching...");

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut matches = 0;

    for line in reader.lines() {
        let line = line.context("failed to read line")?;

        if line.contains(&args.pattern) {
            writeln!(writer, "{}", line)?;
            matches += 1;
        }

        progress_bar.tick();
    }

    writer.flush()?;
    progress_bar.finish_with_message("Search completed!");

    if matches == 0 {
        warn!("no matches found");
    } else {
        info!("found {} matching lines", matches);
    }

    Ok(())
}
