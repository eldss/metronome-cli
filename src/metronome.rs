use std::sync::{
    atomic::{AtomicU32, AtomicU64, Ordering},
    Arc, Mutex,
};

use cpal::traits::StreamTrait;
use fundsp::hacker::Sequencer;

use crate::{audio, config::AppConfig, synth::hihat};

/// Sets up the audio stream and runs the metronome continuously.
pub fn play(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Set up shared state for the audio stream
    let bpm = initialize_bpm(config.bpm);
    let sequence = initialize_sequence(bpm.clone());
    let sample_counter = initialize_sample_counter();

    // Initialize the audio stream and start playback
    let stream =
        audio::initialize_audio_stream(bpm.clone(), sequence.clone(), sample_counter.clone())?;
    stream.play()?;

    // Block until the user presses Enter (the stream runs on the CPAL thread).
    wait_for_user_input();

    Ok(())
}

/// Initializes the BPM as an atomic reference-counted value.
fn initialize_bpm(bpm_value: u32) -> Arc<AtomicU32> {
    Arc::new(AtomicU32::new(bpm_value))
}

/// Initializes the hi-hat sequence as an atomic reference-counted mutex.
fn initialize_sequence(bpm: Arc<AtomicU32>) -> Arc<Mutex<Sequencer>> {
    Arc::new(Mutex::new(hihat::hihat_pattern(
        bpm.load(Ordering::Relaxed),
    )))
}

/// Initializes the sample counter as an atomic reference-counted value.
fn initialize_sample_counter() -> Arc<AtomicU64> {
    Arc::new(AtomicU64::new(0))
}

/// Blocks until the user presses Enter.
fn wait_for_user_input() {
    println!("Press Enter to stop the metronome.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
