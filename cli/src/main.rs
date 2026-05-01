use std::{fs::{self, File}, io::{BufWriter, Write}, path::PathBuf};

use common::{Session};
use rust::plugin::RustPlugin; // manual imports for now

use anyhow::{Result, Context};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
  name = "doqo",
  version,
  about = "Generates static HTML documentation from your code symbols.",
  long_about = None
)]
struct Cli {
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "DIR", default_value = "./docs")]
    pub output: PathBuf,
    #[arg(short = 'n', long, value_name = "PATTERN")]
    pub ignore: Vec<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_level = match cli.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_target(false)
        .init();

    log::debug!("Input directory: {:?}", cli.input);
    log::debug!("Output directory: {:?}", cli.output);
    log::debug!("Ignore patterns: {:?}", cli.ignore);

    let mut session = Session::new(cli.input, &cli.ignore).context("Failed to initialize doqo session.")?;

    let _rust_plugin_id = session.register_plugin(Box::new(RustPlugin)); // manual registering for now

    session.scan_sources().context("Failed to scan sources.")?;

    let json = session.process();
    log::debug!("Registry JSON: \n{}", json);

    let path = fs::canonicalize(cli.output.join("registry.json")).context("Failed to cannonicalize output path.")?;
    log::debug!("Output file path: {}", path.display());

    let file = File::create(&path).context(format!("Failed to create file: {}.", path.display()))?;

    let mut writer = BufWriter::new(file);
    writer.write_all(json.as_bytes()).context(format!("Failed to write JSON to {}.", path.display()))?;
    writer.flush().context("Failed to flush buffer.")?;

    Ok(())
}