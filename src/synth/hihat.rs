use fundsp::prelude::*;

/// Constructs a hi‑hat synth that produces a single 50ms burst.
///
/// Call `reset()` on the returned unit to retrigger the burst.
pub fn hihat_synth() -> Box<dyn AudioUnit> {
    // Burst length in seconds
    let burst_duration = 0.05;
    // Controls exponential decay (higher means faster decay)
    let decay_factor = 60.0;
    // Bandpass center frequency in Hz
    let bp_center = 8000.0;
    // Bandpass Q (resonance factor)
    let bp_q = 1.5;

    // Create a one-shot envelope:
    // For t < burst_duration, amplitude = exp(-t * decay_factor); afterwards, 0.
    let env = envelope(move |t: f32| {
        if t < burst_duration {
            f32::exp(-t * decay_factor)
        } else {
            0.0
        }
    });

    // Compose the hi-hat sound:
    // Multiply white noise by the envelope and then filter with a bandpass.
    Box::new(noise() * env >> bandpass_hz(bp_center, bp_q))
}

/// Creates a new hi-hat pattern and adds it to the given sequencer.
///
/// # Returns
///
/// A vector of `EventId`s representing the events added to the sequencer.
pub fn new_hihat_pattern(
    sequencer: &mut Sequencer,
    bpm: u32,
    drop_beats: Option<(u8, u8)>,
) -> Vec<EventId> {
    let mut event_ids: Vec<EventId> = Vec::new();
    let beat_period = 60.0 / (bpm as f64);

    if let Some((on, off)) = drop_beats {
        let mut beat_start = 0.0;

        // Push on beats
        for _ in 0..on {
            event_ids.push(sequencer.push(
                beat_start,
                beat_start + beat_period,
                Fade::Smooth,
                0.001,
                0.001,
                hihat_synth(),
            ));
            beat_start += beat_period;
        }

        // Push off beats
        for _ in 0..off {
            event_ids.push(sequencer.push(
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
        event_ids.push(sequencer.push(0.0, beat_period, Fade::Smooth, 0.001, 0.001, hihat_synth()));
    }

    event_ids
}
