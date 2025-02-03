use clap::Parser;

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

    /// Custom click type, e.g., "default" or "harmonic"
    #[arg(long, default_value = "default")]
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
    pub rate: Option<f32>,

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
