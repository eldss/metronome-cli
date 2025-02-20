use core::f64;

use crate::helpers;
use fundsp::prelude::*;

/// Generates an electric piano-like synth sound for the given note(s).
///
/// # Arguments
///
/// * `note` - A note strings (e.g., "C4", "E#4", "Gb4").
/// * `duration` - The duration (in seconds) for which the tone should play. If None, the tone will sustain indefinitely.
///
/// # Returns
///
/// An AudioUnit representing the synthesized electric piano tone.
pub fn electric_piano(note: &str, duration: Option<f64>) -> Box<dyn AudioUnit> {
    // Convert note string to frequencies.
    let freq: f32 = helpers::note_to_frequency(note).unwrap_or(0.0);

    // Frequency-dependent loudness compensation (psychoacoustic adjustment)
    // Mid-range reference (A4)
    let reference_freq = 440.0;
    // Prevent division by zero with max, and cap gain
    let gain = (reference_freq / freq.max(1.0)).powf(1.5).min(2.0);

    // Apply the compensation factor to balance volume
    let voice = hammond_hz(freq) * constant(0.015 * gain);

    Box::new(voice)
}

pub fn add_drone_notes(notes: &[String], sequencer: &mut Sequencer) -> Vec<EventId> {
    let mut events: Vec<EventId> = Vec::new();

    for note in notes {
        events.push(sequencer.push(
            0.0,
            f64::INFINITY,
            Fade::Smooth,
            0.001,
            0.001,
            electric_piano(note, None),
        ));
    }

    events
}
