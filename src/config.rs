use crate::cli::CliOptions;

#[derive(Clone)]
pub struct AppConfig {
    pub bpm: u32,
    pub file: Option<String>,
    pub click: String,
    pub drop_beats: Option<(u8, u8)>,
    pub drop_rate: Option<u8>,
    pub ramp: Option<u32>,
    pub rate: Option<f32>,
    pub drone: Option<Vec<String>>,
    pub tones: Option<String>,
    pub progression: Option<String>,
    pub beats_per: Option<Vec<u8>>,
    pub interactive: bool,
    pub record: bool,
    pub analyze: bool,
}

impl AppConfig {
    pub fn from_cli(cli: CliOptions) -> Self {
        // Validate, parse, and organize CLI inputs into AppConfig.
        // For drop_beats, parse the string into a tuple.
        // For drone, split on comma.
        // For beats_per, split and convert to numbers.
        todo!("Parse and create AppConfig from CliOptions")
    }
}
