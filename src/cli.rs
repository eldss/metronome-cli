use clap::{value_parser, Parser};

// TODO: Add validators for all options.

/// CLI options for the metronome application.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliOptions {
    /// Beats per minute
    #[arg(long, value_parser = value_parser!(u32).range(30..301))]
    pub bpm: u32,

    /// Optional custom click sound file
    #[arg(long)]
    pub file: Option<String>,

    /// Custom click type (i.e. "click" or "harmonic")
    #[arg(long, default_value = "click")]
    pub click: String,

    /// Beat dropping pattern as "on,off" (i.e. 4,8) or a single number used for both on and off.
    #[arg(long)]
    pub drop_beats: Option<String>,

    /// Percentage of beats to drop randomly
    #[arg(long, value_parser = value_parser!(u8).range(1..100))]
    pub drop_rate: Option<u8>,

    /// BPM ramp target
    #[arg(long, value_parser = value_parser!(u32).range(30..301))]
    pub ramp: Option<u32>,

    /// BPM change rate (for ramping)
    #[arg(long, value_parser = value_parser!(u8).range(1..11))]
    pub rate: Option<u8>,

    /// Drone tones (comma separated)
    #[arg(long)]
    pub drone: Option<String>,

    /// Tones for harmonic click
    #[arg(long)]
    pub tones: Option<String>,

    /// Chord progression for harmonic click
    #[arg(long)]
    pub progression: Option<String>,

    /// Beats per chord in progression
    #[arg(long)]
    pub beats_per: Option<String>,

    /// Enable interactive BPM adjustments.
    #[arg(long)]
    pub interactive: bool,

    /// Enable recording
    #[arg(long)]
    pub record: bool,

    /// Enable analysis mode
    #[arg(long)]
    pub analyze: bool,
}

impl CliOptions {
    pub fn parse() -> Self {
        // Clap does the parsing.
        clap::Parser::parse()
    }
}
