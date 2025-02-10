mod analysis;
mod audio;
mod cli;
mod config;
mod constants;
mod helpers;
mod metronome;
mod recording;
mod scheduler;
mod synth;
mod terminal;

use cli::CliOptions;
use config::AppConfig;

fn main() {
    // Parse CLI Options
    let cli_options = CliOptions::parse();

    // Convert options into app config
    let config = AppConfig::from_cli(cli_options)
        .map_err(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })
        .unwrap();

    println!("{config:?}");
}
