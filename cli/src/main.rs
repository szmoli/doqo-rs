use std::{
    fs::{self},
    path::PathBuf,
};

use common::Session;
use rust::plugin::RustPlugin; // manual imports for now

use anyhow::{Context, Result};
use clap::Parser;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../frontend/build"]
struct Frontend;

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
    pub verbose: u8,
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

    let mut session =
        Session::new(cli.input, &cli.ignore).context("Failed to initialize doqo session.")?;

    let _rust_plugin_id = session.register_plugin(Box::new(RustPlugin)); // manual registering for now

    session.scan_sources().context("Failed to scan sources.")?;

    let json = session.process();
    log::debug!("Registry JSON: \n{}", json);

    generate_static_site(&cli.output, &json).context("Failed to generate static site.")?;

    Ok(())
}

fn generate_static_site(output_directory: &PathBuf, registry_json: &str) -> Result<()> {
    fs::create_dir_all(output_directory)?;

    for file in Frontend::iter() {
        let file_path = output_directory.join(file.as_ref());

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = Frontend::get(file.as_ref()).unwrap();
        fs::write(file_path, content.data)?;
    }

    let json_path = output_directory.join("registry.json");
    fs::write(json_path, registry_json)?;

    log::info!("Generated static site to {}", output_directory.display());

    Ok(())
}
