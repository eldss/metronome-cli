use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::{
    cli::CliOptions,
    constants::{CHORD_REGEX, NOTE_REGEX},
    helpers,
};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bpm: u32,
    pub drop_beats: Option<(u8, u8)>,
    pub drop_rate: Option<u8>,
    pub ramp: Option<u32>,
    pub change_rate: Option<u8>,
    pub drone: Option<Vec<String>>,
    pub tones: Option<Tones>,
    pub progression: Option<Vec<String>>,
    pub beats_per: Option<Vec<u8>>,
    pub harmonic: bool, // TODO: I don't think I actually need this because I can use tones.is_some().
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tones {
    List(Vec<String>),
    Map(HashMap<String, Vec<String>>),
}

impl AppConfig {
    /// Constructs an AppConfig from the CLI options.
    pub fn from_cli(cli: CliOptions) -> Result<Self, String> {
        // Validate integer arguments.
        let bpm = helpers::validate_and_extract(cli.bpm, 30, 300, "bpm")?;
        let drop_rate = helpers::validate_and_extract_option(cli.drop_rate, 1, 99, "drop-rate")?;
        let ramp = helpers::validate_and_extract_option(cli.ramp, 30, 300, "ramp")?;
        let change_rate =
            helpers::validate_and_extract_option(cli.change_rate, 1, 99, "change-rate")?;

        // Extract list types.
        let drone = Self::get_drone(cli.drone)?;
        let progression = Self::get_progression(cli.progression)?;
        let beats_per = helpers::parse_comma_separated_option::<u8>(cli.beats_per, "beats-per")?;

        // Extract complex types.
        let drop_beats = Self::get_drop_beats(cli.drop_beats)?;
        let tones = Self::get_tones(cli.tones)?;

        let config = AppConfig {
            bpm,
            drop_beats,
            drop_rate,
            ramp,
            change_rate,
            drone,
            tones,
            progression,
            beats_per,
            harmonic: cli.harmonic,
        };

        config.perform_logical_validations()?;
        config.print_warnings();

        Ok(config)
    }

    /// Runs all logical validations. Returns an error if any check fails.
    fn perform_logical_validations(&self) -> Result<(), String> {
        self.no_tones_progression_or_beats_per_if_not_harmonic()?;
        self.progression_and_beats_per_set_if_tones_is_map()?;
        self.no_simultaneous_drop_beats_and_drop_rate()?;
        self.no_drop_beats_or_rate_with_ramp()?;
        self.progression_requires_beats_per()?;
        self.progression_and_beats_per_length_match()?;
        self.progression_and_tones_match()?;
        self.no_simultaneous_drone_and_tones()?;
        Ok(())
    }

    /// Prints warnings to stderr (if any).
    fn print_warnings(&self) {
        self.change_rate_warning();
    }

    fn get_progression(progression: Option<String>) -> Result<Option<Vec<String>>, String> {
        let prog = helpers::parse_comma_separated_option::<String>(progression, "progression")?;
        match prog {
            Some(list) => {
                if list.len() > 24 {
                    return Err("Progression must be at most 24 chords long".to_string());
                }
                Ok(Some(list))
            }
            None => Ok(None),
        }
    }

    fn get_drone(drone: Option<String>) -> Result<Option<Vec<String>>, String> {
        match drone {
            Some(list) => {
                let note_re = Regex::new(&format!("^{}", NOTE_REGEX))
                    .map_err(|e| format!("Invalid note regex: {}", e))?;
                let parsed_list: Vec<String> = helpers::parse_comma_separated(&list, "drone")?;

                if parsed_list.len() > 4 {
                    return Err("Note list must contain between 1 and 4 notes.".to_string());
                }

                for note in &parsed_list {
                    if !note_re.is_match(&note) {
                        return Err(format!(
                            "Invalid note format for {}. Expected format: <note><octave> where <note> is a letter A-G, followed by an optional # or b, and <octave> is a number 1-6",
                            note
                        ));
                    }
                }
                Ok(Some(parsed_list))
            }
            None => Ok(None),
        }
    }

    /// Gets the tones parameter and returns it as a Tones enum.
    fn get_tones(tones: Option<String>) -> Result<Option<Tones>, String> {
        match tones {
            Some(list) => {
                let chord_re = Regex::new(&format!("^{}", CHORD_REGEX))
                    .map_err(|e| format!("Invalid chord regex: {}", e))?;
                let note_re = Regex::new(&format!("^{}", NOTE_REGEX))
                    .map_err(|e| format!("Invalid note regex: {}", e))?;
                let parsed_list = helpers::parse_comma_separated(&list, "tones")?;

                if chord_re.is_match(&list) {
                    if parsed_list.len() > 10 {
                        return Err("Chord list must contain between 1 and 10 chords.".to_string());
                    }
                    let map = Self::build_tone_map(&parsed_list, chord_re, note_re)?;
                    Ok(Some(Tones::Map(map)))
                } else if note_re.is_match(&list) {
                    if parsed_list.len() > 4 {
                        return Err("Note list must contain between 1 and 4 notes.".to_string());
                    }
                    // Above only checks start of string. Need to check all notes.
                    for note in &parsed_list {
                        if !note_re.is_match(&note) {
                            return Err(format!(
                                "Invalid note format for {}. Expected format: <note><octave> where <note> is a letter A-G, followed by an optional # or b, and <octave> is a number 1-6",
                                note
                            ));
                        }
                    }
                    Ok(Some(Tones::List(parsed_list)))
                } else {
                    Err(format!("Received an unexpected value for tones: {list}."))
                }
            }
            None => Ok(None),
        }
    }

    /// For each item in the given list, extracts a chord ID for a HashMap key, then extracts chord tones for the value.
    /// Expects a specific formatting for the items or returns an error.
    fn build_tone_map(
        list: &Vec<String>,
        chord_regex: Regex,
        note_regex: Regex,
    ) -> Result<HashMap<String, Vec<String>>, String> {
        let mut chord_map: HashMap<String, Vec<String>> = HashMap::new();

        for chord in list {
            if !chord_regex.is_match(chord) {
                return Err(format!(
                    "Invalid chord format for {}. Expected format: <ID>(<notes>) where <ID> is made of alphanumeric characters and special characters _ + - #",
                    chord
                ));
            }
            let parts: Vec<&str> = chord.split(['(', ')']).collect();
            if parts.len() != 3 {
                return Err(format!(
                    "Invalid chord format for {}. Expected format: <ID>(<notes>) where <ID> is made of alphanumeric characters and special characters _ + - #",
                    chord
                ));
            }
            let id = parts[0].trim().to_string();
            let notes: Result<Vec<String>, String> = parts[1]
                .split_whitespace()
                .map(|note| {
                    if !note_regex.is_match(note) {
                        Err(format!(
                            "Invalid note format for {}. Expected format: <note><octave> where <note> is a letter A-G, followed by an optional # or b, and <octave> is a number 1-6",
                            note
                        ))
                    } else {
                        Ok(note.to_string())
                    }
                })
                .collect();

            let notes = notes?;
            if notes.len() > 4 {
                return Err("Note list must contain between 1 and 4 notes.".to_string());
            }

            chord_map.insert(id, notes);
        }
        Ok(chord_map)
    }

    /// Gets the drop_beats parameter.
    fn get_drop_beats(dropped: Option<String>) -> Result<Option<(u8, u8)>, String> {
        let param_name = "drop-beats";
        match dropped {
            Some(val) => {
                let parts: Vec<u8> = helpers::parse_comma_separated(&val, param_name)?;
                match parts.len() {
                    1 => Ok(Some((parts[0], parts[0]))),
                    2 => Ok(Some((parts[0], parts[1]))),
                    num => Err(format!(
                        "Invalid number of values for {}: {}. Expected 1 or 2.",
                        param_name, num
                    )),
                }
            }
            None => Ok(None),
        }
    }

    fn no_tones_progression_or_beats_per_if_not_harmonic(&self) -> Result<(), String> {
        if !self.harmonic
            && (self.tones.is_some() || self.progression.is_some() || self.beats_per.is_some())
        {
            Err("Cannot set tones, progression, or beats-per if click is not harmonic.".to_string())
        } else {
            Ok(())
        }
    }

    fn progression_and_beats_per_set_if_tones_is_map(&self) -> Result<(), String> {
        if let Some(Tones::Map(_)) = &self.tones {
            if self.progression.is_none() || self.beats_per.is_none() {
                return Err(
                    "If tones is a map, progression and beats-per must also be set.".to_string(),
                );
            }
        }
        Ok(())
    }

    fn no_simultaneous_drop_beats_and_drop_rate(&self) -> Result<(), String> {
        if self.drop_beats.is_some() && self.drop_rate.is_some() {
            Err(
                "Cannot set both drop-beats and drop-rate. Please choose one or the other."
                    .to_string(),
            )
        } else {
            Ok(())
        }
    }

    fn no_drop_beats_or_rate_with_ramp(&self) -> Result<(), String> {
        if (self.drop_beats.is_some() || self.drop_rate.is_some()) && self.ramp.is_some() {
            Err("Cannot drop beats if ramp is set. Please choose one or the other.".to_string())
        } else {
            Ok(())
        }
    }

    fn progression_requires_beats_per(&self) -> Result<(), String> {
        if self.progression.is_some() && self.beats_per.is_none() {
            Err(
                "If progression is set, beats-per must also be set. Please set beats-per."
                    .to_string(),
            )
        } else {
            Ok(())
        }
    }

    fn progression_and_beats_per_length_match(&self) -> Result<(), String> {
        if let (Some(progression), Some(beats_per)) = (&self.progression, &self.beats_per) {
            if beats_per.len() != 1 && progression.len() != beats_per.len() {
                return Err(
                    "If progression is set, beats-per must be the same length, or a single number."
                        .to_string(),
                );
            }
        }
        Ok(())
    }

    fn progression_and_tones_match(&self) -> Result<(), String> {
        if let (Some(progression), Some(Tones::Map(tones))) = (&self.progression, &self.tones) {
            let tone_keys: HashSet<&String> = tones.keys().collect();
            let prog_keys: HashSet<&String> = progression.iter().collect();
            if tone_keys != prog_keys {
                return Err("If progression is set, tones should represent chords matching the progression.".to_string());
            }
        }
        Ok(())
    }

    fn no_simultaneous_drone_and_tones(&self) -> Result<(), String> {
        if self.drone.is_some() && self.tones.is_some() {
            Err("Cannot set both drone and tones. Please choose one or the other.".to_string())
        } else {
            Ok(())
        }
    }

    fn change_rate_warning(&self) {
        if self.change_rate.is_some() && self.ramp.is_none() {
            eprintln!("Warning: change-rate is set but ramp is not. change-rate will be ignored.");
        }
    }
}

mod tests {
    use super::AppConfig;
    use crate::cli::CliOptions;
    use rstest::{fixture, rstest};

    #[fixture]
    fn base_cli() -> CliOptions {
        CliOptions {
            bpm: 120,
            drop_beats: None,
            drop_rate: None,
            ramp: None,
            change_rate: None,
            drone: None,
            tones: None,
            progression: None,
            beats_per: None,
            harmonic: false,
        }
    }

    #[rstest]
    fn bpm_is_a_number(base_cli: CliOptions) {
        let config = AppConfig::from_cli(base_cli).unwrap();
        assert_eq!(config.bpm, 120);
    }

    #[rstest]
    fn drop_beats_is_a_tuple_given_two_nums(base_cli: CliOptions) {
        let cli = CliOptions {
            drop_beats: Some(String::from("4,8")),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(config.drop_beats, Some((4, 8)));
    }

    #[rstest]
    fn drop_beats_is_a_tuple_given_one_num(base_cli: CliOptions) {
        let cli = CliOptions {
            drop_beats: Some(String::from("4")),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(config.drop_beats, Some((4, 4)));
    }

    #[rstest]
    fn drop_rate_is_a_number(base_cli: CliOptions) {
        let cli = CliOptions {
            drop_rate: Some(50),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(config.drop_rate, Some(50));
    }

    #[rstest]
    fn ramp_is_a_number(base_cli: CliOptions) {
        let cli = CliOptions {
            ramp: Some(150),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(config.ramp, Some(150));
    }

    #[rstest]
    fn change_rate_is_a_number(base_cli: CliOptions) {
        let cli = CliOptions {
            change_rate: Some(50),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(config.change_rate, Some(50));
    }

    #[rstest]
    #[case("A2", vec!["A2"])]
    #[case("A#5,Ab2,Bb3,C#4", vec!["A#5", "Ab2", "Bb3", "C#4"])]
    fn drone_is_a_vec_of_strings(
        base_cli: CliOptions,
        #[case] drone: &str,
        #[case] expected: Vec<&str>,
    ) {
        let cli = CliOptions {
            drone: Some(String::from(drone)),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(
            config.drone,
            Some(expected.iter().map(|s| s.to_string()).collect())
        );
    }

    #[rstest]
    #[case("H")]
    #[case("A3,B3,C3,Z3")]
    #[case("A3,B3,C3,H3")]
    #[case("A3,B3,C3,D3,E3")]
    #[case("A,B3,C3")]
    #[case("A&")]
    #[case("Ab9")]
    fn get_drone_fails_if_note_is_invalid(base_cli: CliOptions, #[case] drone: &str) {
        let cli = CliOptions {
            drone: Some(String::from(drone)),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        println!("{:?}", config);
        assert!(config.is_err());
    }

    #[rstest]
    #[case("A2", vec!["A2"])]
    #[case("A#5,Ab2,Bb3,C#4", vec!["A#5", "Ab2", "Bb3", "C#4"])]
    fn tones_list_is_a_vec_of_strings(
        base_cli: CliOptions,
        #[case] tones: &str,
        #[case] expected: Vec<&str>,
    ) {
        let cli = CliOptions {
            tones: Some(String::from(tones)),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(
            config.tones,
            Some(super::Tones::List(
                expected.iter().map(|s| s.to_string()).collect()
            ))
        );
    }

    #[rstest]
    #[case("H")]
    #[case("A3,B3,C3,Z3")]
    #[case("A3,B3,C3,H3")]
    #[case("A3,B3,C3,G3,F3")]
    #[case("A,B3,C3")]
    #[case("A&")]
    #[case("Ab9")]
    fn tones_list_fails_on_invalid_input(base_cli: CliOptions, #[case] tones: &str) {
        let cli = CliOptions {
            tones: Some(String::from(tones)),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    #[case("Cmaj(A5 B2 C3)", vec!["Cmaj"], vec![vec!["A5", "B2", "C3"]], "Cmaj")]
    #[case("Cmaj(A5 B2 C3),yyy(Ab3 G#2)", vec!["Cmaj", "yyy"], vec![vec!["A5", "B2", "C3"], vec!["Ab3", "G#2"]], "Cmaj,yyy,Cmaj")]
    fn tones_map_is_a_hashmap_of_strings(
        base_cli: CliOptions,
        #[case] tones: &str,
        #[case] expected_keys: Vec<&str>,
        #[case] expected_values: Vec<Vec<&str>>,
        #[case] progression: &str,
    ) {
        let cli = CliOptions {
            tones: Some(String::from(tones)),
            beats_per: Some(String::from("4")),
            progression: Some(String::from(progression)),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        let expected_map: std::collections::HashMap<String, Vec<String>> = expected_keys
            .iter()
            .zip(expected_values.iter())
            .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
            .collect();
        assert_eq!(config.tones, Some(super::Tones::Map(expected_map)));
    }

    #[rstest]
    #[case("12345678910(A1 B2 C3)", "12345678910")]
    #[case("{(A1 B2 C3)", "{")]
    #[case("a (A1 B2 C3)", "a ")]
    fn invalid_tone_map_keys_fail(
        base_cli: CliOptions,
        #[case] tones: &str,
        #[case] progression: &str,
    ) {
        let cli = CliOptions {
            tones: Some(String::from(tones)),
            progression: Some(String::from(progression)),
            beats_per: Some(String::from("4")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    #[case("A(H)")]
    #[case("A(A3,B3,C3,Z3)")]
    #[case("A(A3,B3,C3,C2,C1)")]
    #[case("A(H3,B3,C3)")]
    #[case("A(A4,B,C3)")]
    #[case("A(A&)")]
    #[case("A(Ab9)")]
    #[case("A(A3),B(B3),C(C3),D(D3),E(E3),F(F3),G(G3)A(G3),B(F3),C(E3),D(D3),E(D3),F(B3),G(A3)")]
    fn invalid_tone_map_values_fail(base_cli: CliOptions, #[case] tones: &str) {
        let cli = CliOptions {
            tones: Some(String::from(tones)),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn progression_works_with_beats_per_same_length(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from("Cmaj,Dmin,E7")),
            beats_per: Some(String::from("4,3,2")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(
            config.progression,
            Some(
                vec!["Cmaj", "Dmin", "E7"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(config.beats_per, Some(vec![4, 3, 2]));
    }

    #[rstest]
    fn progression_fails_if_len_over_24(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from(
                "1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1",
            )),
            tones: Some(String::from("1(A3),2(B3)")),
            beats_per: Some(String::from("4")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn progression_works_with_beats_per_single_value(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from("Cmaj,Dmin,E7")),
            beats_per: Some(String::from("4")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli).unwrap();
        assert_eq!(
            config.progression,
            Some(
                vec!["Cmaj", "Dmin", "E7"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(config.beats_per, Some(vec![4]));
    }

    #[rstest]
    fn progression_and_beats_per_length_mismatch(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from("Cmaj,Dmin,E7")),
            beats_per: Some(String::from("4,3")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn progression_and_tones_mismatch(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from("Cmaj,Dmin,E7")),
            tones: Some(String::from("Cmaj(A1 B2 C3),Dmin(Ab3 G#2)")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn no_tones_progression_or_beats_per_if_not_harmonic(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from("Cmaj,Dmin,E7")),
            beats_per: Some(String::from("4,3,2")),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn no_simultaneous_drop_beats_and_drop_rate(base_cli: CliOptions) {
        let cli = CliOptions {
            drop_beats: Some(String::from("4,8")),
            drop_rate: Some(50),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn no_drop_beats_or_rate_with_ramp(base_cli: CliOptions) {
        let cli = CliOptions {
            drop_beats: Some(String::from("4,8")),
            ramp: Some(150),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn progression_requires_beats_per(base_cli: CliOptions) {
        let cli = CliOptions {
            progression: Some(String::from("Cmaj,Dmin,E7")),
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }

    #[rstest]
    fn no_simultaneous_drone_and_tones(base_cli: CliOptions) {
        let cli = CliOptions {
            drone: Some(String::from("A1")),
            tones: Some(String::from("Cmaj(A1 B2 C3)")),
            harmonic: true,
            ..base_cli
        };
        let config = AppConfig::from_cli(cli);
        assert!(config.is_err());
    }
}
