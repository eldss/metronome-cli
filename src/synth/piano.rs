use core::f64;

use crate::helpers;
use fundsp::prelude::*;

/// Generates an electric piano-like synth sound for the given note(s).
///
/// # Arguments
///
/// * `note` - A note string (e.g., "C4", "E#4", "Gb4").
/// * `duration` - The duration (in seconds) for which the tone should play. If None, the tone will sustain indefinitely.
/// * `num_total_notes` - The total number of notes in the chord/sequence played together.
///
/// # Returns
///
/// An AudioUnit representing the synthesized electric piano tone.
pub fn electric_piano(
    note: &str,
    duration: Option<f32>,
    num_total_notes: usize,
) -> Box<dyn AudioUnit> {
    // Convert note string to frequency.
    let freq: f32 = helpers::note_to_frequency(note).unwrap_or(0.0);
    let voice = hammond_hz(freq) * constant(0.025) >> lowpass_hz(1000.0, 1.0);

    // Frequency correction: use a reference (say, C4 = 261.63 Hz)
    // Lower notes (smaller freq) get a boost so that C2 sounds as loud as C4–C5.
    let freq_gain = frequency_correction(freq);

    // When multiple voices are mixed, scale the output.
    let exponent = 0.3;
    let mix_gain = 1.0 / (num_total_notes as f32).powf(exponent);

    let voice = voice * freq_gain * mix_gain;

    if let Some(dur) = duration {
        // Envelope normalization:
        // Our envelope is defined as exp(-t * decay_factor) for t < dur.
        // Its energy (squared amplitude integrated over the duration) is:
        //    energy = ∫₀^(dur) exp(-2*t*decay_factor) dt
        // and its RMS over the duration is sqrt(energy / dur).
        // We then compute a factor that brings that RMS to 1.
        let decay_factor = 10.0;
        let energy = (1.0 - (-2.0 * decay_factor * dur).exp()) / (2.0 * decay_factor);
        let rms = (energy / dur).sqrt();
        let env_gain = if rms > 0.0 { 1.0 / rms } else { 1.0 };

        // Create a one-shot envelope with a sine-shaped attack:
        // For t < attack_time, amplitude = sin( (t/attack_time) * (pi/2) );
        // For t between attack_time and burst_duration, amplitude = exp( - (t - attack_time) * decay_factor );
        // Otherwise, output 0.
        let attack_time = 0.001;
        let env = envelope(move |t: f32| {
            if t < attack_time {
                // Sine ramp from 0 to 1.
                (t / attack_time * std::f32::consts::FRAC_PI_2).sin()
            } else if t < dur {
                f32::exp(-(t - attack_time) * decay_factor)
            } else {
                0.0
            }
        });

        Box::new(voice * env * env_gain)
    } else {
        Box::new(voice)
    }
}

/// Returns a frequency correction factor based on the note frequency.
/// Boosts low frequencies more aggressively (using a power law)
/// but clamps the maximum boost to avoid blowing out the speakers.
fn frequency_correction(freq: f32) -> f32 {
    // reference: C4
    let ref_freq = 261.63;

    // For frequencies below 200 Hz, apply additional boost.
    if freq < 200.0 {
        // Here we boost using (200 / freq) raised to an exponent.
        // You can experiment with the exponent (e.g. 0.7) to get the desired boost.
        let boost = (200.0 / freq).powf(0.7);
        // Clamp to a maximum gain of, say, 3.0
        boost.min(3.0)
    } else {
        // For higher frequencies, you might still want a mild correction,
        // so we use a milder power law relative to ref_freq.
        (ref_freq / freq).powf(0.6)
    }
}

/// Adds a series of drone notes to the sequencer.
///
/// # Arguments
///
/// * `notes` - A slice of note strings (e.g., "C4", "E#4", "Gb4").
/// * `sequencer` - A mutable reference to the sequencer to which the notes should be added.
///
/// # Returns
///
/// A vector of `EventId`s representing the events added to the sequencer.
pub fn add_drone_notes(notes: &[String], sequencer: &mut Sequencer) -> Vec<EventId> {
    let mut events: Vec<EventId> = Vec::new();

    for note in notes {
        events.push(sequencer.push(
            0.0,
            f64::INFINITY,
            Fade::Smooth,
            0.001,
            0.001,
            electric_piano(note, None, notes.len()),
        ));
    }

    events
}

/// Adds a series of notes to the sequencer at regular intervals.
/// The notes will play for the specified duration and be spaced by the beat duration.
///
/// # Arguments
///
/// * `notes` - A slice of note strings (e.g., "C4", "E#4", "Gb4").
/// * `sequencer` - A mutable reference to the sequencer to which the notes should be added.
/// * `note_duration` - The duration (in seconds) for which each note should play.
/// * `bpm` - The beats per minute for the sequencer.
/// * `drop_beats` - An optional tuple of two u8 values representing the number of on and off beats to drop.
///
/// # Returns
///
/// A vector of `EventId`s representing the events added to the sequencer.
pub fn add_time_notes(
    notes: &[String],
    sequencer: &mut Sequencer,
    note_duration: f32,
    bpm: u32,
    drop_beats: Option<(u8, u8)>,
) -> Vec<EventId> {
    let mut events: Vec<EventId> = Vec::new();
    let beat_period = 60.0 / (bpm as f64);

    if let Some((on, off)) = drop_beats {
        let mut beat_start = 0.0;

        // Push on beats
        for _ in 0..on {
            for note in notes {
                events.push(sequencer.push(
                    beat_start,
                    beat_start + beat_period,
                    Fade::Smooth,
                    0.001,
                    0.001,
                    electric_piano(note, Some(note_duration), notes.len()),
                ));
            }
            beat_start += beat_period;
        }

        // Push off beats
        for _ in 0..off {
            events.push(sequencer.push(
                beat_start,
                beat_start + beat_period,
                Fade::Smooth,
                0.001,
                0.001,
                Box::new(zero()),
            ));
            beat_start += beat_period;
        }
    } else {
        for note in notes {
            events.push(sequencer.push(
                0.0,
                beat_period,
                Fade::Smooth,
                0.001,
                0.001,
                electric_piano(note, Some(note_duration), notes.len()),
            ));
        }
    }

    events
}
