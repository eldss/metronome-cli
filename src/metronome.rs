use std::sync::{
    atomic::{AtomicU32, AtomicU64},
    Arc, Mutex,
};

use cpal::traits::StreamTrait;

use crate::{audio, config::AppConfig, synth};

pub struct Metronome {
    /// Shared adjustable bpm
    bpm: Arc<AtomicU32>,
    /// Shared synth to create sounds
    synth: Arc<Mutex<synth::Synth>>,
    /// Shared counter to determine when to reset synth
    sample_counter: Arc<AtomicU64>,
}

impl Metronome {
    pub fn new(config: &AppConfig) -> Self {
        let bpm = Arc::new(AtomicU32::new(config.bpm));
        let synth = Arc::new(Mutex::new(synth::Synth::from(config)));
        let sample_counter = Arc::new(AtomicU64::new(0));

        Metronome {
            bpm,
            synth,
            sample_counter,
        }
    }

    /// Sets up the audio stream and runs the metronome continuously.
    pub fn play(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let stream = audio::initialize_audio_stream(
            self.bpm.clone(),
            self.synth.clone(),
            self.sample_counter.clone(),
            config,
        )?;
        stream.play()?;

        wait_for_user_input();

        Ok(())
    }
}

/// Blocks until the user presses Enter.
fn wait_for_user_input() {
    println!("Press Enter to stop the metronome.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
