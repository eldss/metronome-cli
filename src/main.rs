mod audio;
mod cli;
mod config;
mod constants;
mod helpers;
mod metronome;
mod synth;
mod terminal;

use cli::CliOptions;
use config::AppConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI Options
    let cli_options = CliOptions::parse();

    // Convert options into app config
    let config = AppConfig::from_cli(cli_options)?;

    metronome::play(&config)?;

    Ok(())
}
