use core::f64;

use fundsp::prelude::*;

use crate::config::AppConfig;

pub mod hihat;
pub mod piano;

pub struct Synth {
    pub sequencer: Sequencer,
    pub hihat_events: Vec<EventId>,
    _piano_events: Vec<EventId>,
}

impl Synth {
    pub fn from(config: &AppConfig) -> Self {
        let mut sequencer = Sequencer::new(true, 1);
        let hihat_events = hihat::new_hihat_pattern(&mut sequencer, config.bpm, config.drop_beats);
        let _piano_events =
            piano::add_drone_notes(config.drone.as_deref().unwrap_or(&[]), &mut sequencer);
        Synth {
            sequencer,
            hihat_events,
            _piano_events,
        }
    }
}
