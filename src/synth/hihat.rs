use fundsp::prelude::*;

/// Constructs a hi‑hat synth that produces a single 50ms burst.
///
/// Call `reset()` on the returned unit to retrigger the burst.
pub fn hi_hat_synth() -> impl AudioUnit {
    // Burst length in seconds (50ms)
    let burst_duration = 0.05;
    // Controls exponential decay (higher means faster decay)
    let decay_factor = 60.0;
    // Bandpass center frequency in Hz
    let bp_center = 8000.0;
    // Bandpass Q (resonance factor)
    let bp_q = 1.0;

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
    noise() * env >> bandpass_hz(bp_center, bp_q)
}

/// Returns a Sequencer that plays a hi‑hat hit (using your hi_hat_synth)
/// at every beat. The BPM is provided as a u32 and determines the beat period.
///
/// In this example, the hi‑hat event is scheduled from time 0.0 to (60 / BPM) seconds.
/// Since `hi_hat_synth` outputs silence after its 50ms burst, the remaining time is silent.
pub fn hihat_pattern(bpm: u32) -> Sequencer {
    // Calculate beat period in seconds (60 seconds per minute)
    let beat_period = 60.0 / (bpm as f64);

    // Create a Sequencer that will replay its events when reset.
    // We use 1 output channel (mono).
    let mut sequencer = Sequencer::new(true, 1);

    // Push a hi‑hat event that lasts for the full beat.
    // Even though the hi_hat_synth only produces sound for 50ms,
    // scheduling it for the entire beat allows for easy extension (syncopations, etc.).
    let _event_id = sequencer.push(
        0.0,          // start time (seconds)
        beat_period,  // end time (seconds)
        Fade::Smooth, // fade type for smooth transitions
        0.001,        // fade-in duration (seconds)
        0.001,        // fade-out duration (seconds)
        Box::new(hi_hat_synth()),
    );

    // When processing the audio graph, you must call `reset()` on the sequencer
    // every beat (or whenever BPM changes) so that the event re-triggers.
    sequencer
}
