use clap::Parser;

// TODO: Add validators for all options.

/// CLI options for the metronome application.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliOptions {
    /// Beats per minute
    #[arg(long)]
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
    #[arg(long)]
    pub drop_rate: Option<u8>,

    /// BPM ramp target
    #[arg(long)]
    pub ramp: Option<u32>,

    /// BPM change rate (for ramping)
    #[arg(long)]
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
    #[arg(long, default_value_t = false)]
    pub interactive: bool,

    /// Enable recording
    #[arg(long, default_value_t = false)]
    pub record: bool,

    /// Enable analysis mode
    #[arg(long, default_value_t = false)]
    pub analyze: bool,
}

impl CliOptions {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
}
