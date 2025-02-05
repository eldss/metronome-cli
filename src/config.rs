use std::{collections::HashMap, fmt::Display, str::FromStr};

use regex::Regex;

use crate::{
    cli::CliOptions,
    constants::{CHORD_REGEX, NOTE_REGEX},
};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bpm: u32,
    pub file: Option<String>,
    pub click: String,
    pub drop_beats: Option<(u8, u8)>,
    pub drop_rate: Option<u8>,
    pub ramp: Option<u32>,
    pub rate: Option<u8>,
    pub drone: Option<Vec<String>>,
    pub tones: Option<Tones>,
    pub progression: Option<Vec<String>>,
    pub beats_per: Option<Vec<u8>>,
    pub interactive: bool,
    pub record: bool,
    pub analyze: bool,
}

#[derive(Clone, Debug)]
pub enum Tones {
    List(Vec<String>),
    Map(HashMap<String, Vec<String>>),
}

impl AppConfig {
    pub fn from_cli(cli: CliOptions) -> Self {
        // Validate integer arguments
        let bpm = Self::validate_in_range(cli.bpm, 30, 300, "bpm");
        let drop_rate = Self::validate_opt_in_range(cli.drop_rate, 1, 99, "drop-rate");
        let ramp = Self::validate_opt_in_range(cli.ramp, 30, 300, "ramp");
        let rate = Self::validate_opt_in_range(cli.rate, 1, 15, "rate");

        // Extract list types
        let drone = Self::opt_string_to_vec(cli.drone, "drone");
        let progression = Self::opt_string_to_vec(cli.progression, "progression");
        let beats_per = Self::opt_string_to_vec(cli.beats_per, "beats-per");

        // Extract complex types
        let drop_beats = Self::get_drop_beats(cli.drop_beats);
        let tones = Self::get_tones(cli.tones);

        AppConfig {
            bpm,
            file: cli.file,
            click: cli.click,
            drop_beats,
            drop_rate,
            ramp,
            rate,
            drone,
            tones,
            progression,
            beats_per,
            interactive: cli.interactive,
            record: cli.record,
            analyze: cli.analyze,
        }
    }

    /// Gets the tones parameter.
    fn get_tones(tones: Option<String>) -> Option<Tones> {
        match tones {
            Some(list) => {
                let chord_re =
                    Regex::new(&format!("^{}", CHORD_REGEX)).expect("regex should be valid");
                let note_re =
                    Regex::new(&format!("^{}", NOTE_REGEX)).expect("regex should be valid");
                let parsed_list: Vec<String> = Self::parse_comma_separated(&list, "tones");

                if chord_re.is_match(&list) {
                    let map = Self::build_tone_map(&parsed_list, chord_re, note_re);
                    Some(Tones::Map(map))
                } else if note_re.is_match(&list) {
                    Some(Tones::List(parsed_list))
                } else {
                    panic!("Received an unexpected value for tones: {}.", list)
                }
            }
            None => None,
        }
    }

    /// For each item in the given list, extracts a chord ID for a HashMap key, then extracts chord tones for the value.
    /// Expects specific formatting for the items in the list or halts the program. Returns the HashMap.
    fn build_tone_map(
        list: &Vec<String>,
        chord_regex: Regex,
        note_regex: Regex,
    ) -> HashMap<String, Vec<String>> {
        let mut chord_map: HashMap<String, Vec<String>> = HashMap::new();

        for chord in list {
            if !chord_regex.is_match(&chord) {
                panic!(
                    "Invalid chord format for {chord}. Expected format: <ID>(<notes>) {}",
                    "where <ID> is made of alphanumeric characters and special characters _ + - #"
                );
            }
            let parts: Vec<&str> = chord.split(['(', ')']).collect();
            if parts.len() != 3 {
                panic!(
                    "Invalid chord format for {chord}. Expected format: <ID>(<notes>) {}",
                    "where <ID> is made of alphanumeric characters and special characters _ + - #"
                );
            }
            let id = parts[0].trim().to_string();
            let notes: Vec<String> = parts[1]
                .split_whitespace()
                .map(|note| {
                    if !note_regex.is_match(note) {
                        panic!(
                            "Invalid note format: {note}. {}. {}.",
                            "Note is a capital letter from A to G, optionally followed by # or b, and an octave number from 1 to 6.",
                            "For reference, middle C is C3."
                        );
                    }
                    note.to_string()
                })
                .collect();

            chord_map.insert(id, notes);
        }
        chord_map
    }

    /// Gets the drop_beats parameter.
    fn get_drop_beats(dropped: Option<String>) -> Option<(u8, u8)> {
        let param_name = "drop-beats";
        match dropped {
            Some(val) => {
                let parts: Vec<u8> = Self::parse_comma_separated(&val, param_name);
                match parts.len() {
                    1 => Some((parts[0], parts[0])),
                    2 => Some((parts[0], parts[1])),
                    num => panic!(
                        "Wrong number of values for {}. Given {}, expected 1 or 2.",
                        param_name, num
                    ),
                }
            }
            None => None,
        }
    }

    /// Checks if the given value is within an inclusive range and panics with a useful error message if not.
    fn validate_in_range<T: PartialOrd + Display + Copy>(
        val: T,
        low: T,
        high: T,
        param_name: &str,
    ) -> T {
        if !(low..=high).contains(&val) {
            panic!("Invalid value for {param_name}: {val} is outside the range [{low}, {high}].");
        }
        val
    }

    /// Checks if the given Option is within an inclusive range and panics with a useful error message if not.
    fn validate_opt_in_range<T: PartialOrd + Display + Copy>(
        opt: Option<T>,
        low: T,
        high: T,
        param_name: &str,
    ) -> Option<T> {
        if let Some(val) = opt {
            Self::validate_in_range(val, low, high, param_name);
        }
        opt
    }

    /// Converts an comma separated list of numbers as a string in an Option to a vector of numbers in an Option.
    fn opt_string_to_vec<T>(opt: Option<String>, param_name: &str) -> Option<Vec<T>>
    where
        T: FromStr + Display,
        T::Err: Display,
    {
        match opt {
            Some(list) => Some(Self::parse_comma_separated(&list, param_name)),
            None => None,
        }
    }

    /// Given a string, converts it into a vector of the values separated by commas.
    fn parse_comma_separated<T>(val: &str, param_name: &str) -> Vec<T>
    where
        T: FromStr + Display,
        T::Err: Display,
    {
        val.split(',')
            .map(|num_str| {
                num_str.parse::<T>().unwrap_or_else(|err| {
                    panic!(
                        "Problem parsing value given for {}: {}. {}",
                        param_name, num_str, err
                    );
                })
            })
            .collect()
    }
}
