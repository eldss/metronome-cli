use fundsp::prelude::*;

use crate::config::{AppConfig, Tones};

pub mod hihat;
pub mod piano;

pub struct Synth {
    pub sequencer: Sequencer,
    _time_events: Vec<EventId>,
    _drone_events: Vec<EventId>,
}

impl Synth {
    pub fn from(config: &AppConfig) -> Self {
        let mut sequencer = Sequencer::new(true, 1);

        // Time events are the metronome click. They can be hihat or piano notes.
        let _time_events = match &config.tones {
            Some(tone_enum) => match tone_enum {
                // Harmonic metronome with unchanging tones
                Tones::List(tone_list) => piano::add_time_notes(
                    tone_list,
                    &mut sequencer,
                    0.2,
                    config.bpm,
                    config.drop_beats,
                ),

                // Harmonic metronome with a changing chord progression.
                Tones::Map(tone_map) => {
                    // TODO: Handle map case
                    piano::add_time_notes(
                        &tone_map.keys().cloned().collect::<Vec<String>>(),
                        &mut sequencer,
                        0.1,
                        config.bpm,
                        config.drop_beats,
                    )
                }
            },
            // Tones were not given, so a valid CLI invocation must mean we are not in harmonic mode.
            None => hihat::new_hihat_pattern(&mut sequencer, config.bpm, config.drop_beats),
        };

        // Drone notes play continuously. They are not allowed in harmonic mode at this time.
        let _drone_events = if config.harmonic {
            vec![]
        } else {
            piano::add_drone_notes(config.drone.as_deref().unwrap_or(&[]), &mut sequencer)
        };

        Synth {
            sequencer,
            _time_events,
            _drone_events,
        }
    }
}
