use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, SampleFormat, Stream, StreamConfig,
};
use fundsp::prelude::*;
use std::{
    error::Error,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use crate::synth;

/// Initializes the audio host, selects the default output device, and builds an output stream.
///
/// # Arguments
///
/// * `bpm` - An `Arc` pointing to an `AtomicU32` representing the beats per minute.
/// * `sequencer` - An `Arc` pointing to a `Mutex`-wrapped `Sequencer`.
/// * `sample_counter` - An `Arc` pointing to an `AtomicU64` for tracking the sample count.
///
/// # Returns
///
/// * `Ok(Stream)` - The configured output audio stream ready for playback.
/// * `Err(Box<dyn Error>)` - An error if the stream couldn't be created.
pub fn initialize_audio_stream(
    bpm: Arc<AtomicU32>,
    synth: Arc<Mutex<synth::Synth>>,
    sample_counter: Arc<AtomicU64>,
) -> Result<Stream, Box<dyn Error>> {
    let device = get_audio_device()?;
    let config = get_stream_config(&device)?;

    // Extract the sample rate as a f64 for calculations and build the output stream
    let sample_rate = config.sample_rate.0 as f64;
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Lock the sequencer for thread-safe access.
            let mut synth_lock = match synth.lock() {
                Ok(lock) => lock,
                Err(poisoned) => {
                    eprintln!("Failed to lock sequencer: {:?}", poisoned);
                    return;
                }
            };

            // Calculate the number of samples per beat.
            let current_bpm = bpm.load(Ordering::Relaxed);
            let beat_period = 60.0 / (current_bpm as f64);
            let num_beats_in_cycle = synth_lock.hihat_events.len() as f64;

            let seq_samples = (beat_period * sample_rate * num_beats_in_cycle).round() as u64;

            // Process each frame in the output buffer.
            for frame in data.chunks_mut(config.channels as usize) {
                // Retrieve the next sample from the sequencer.
                let sample = synth_lock.sequencer.get_mono();
                for sample_out in frame.iter_mut() {
                    *sample_out = sample as f32;
                }

                // Update the sample counter and reset the sequencer if a beat has completed.
                let prev_count = sample_counter.fetch_add(1, Ordering::Relaxed) + 1;
                if prev_count >= seq_samples {
                    synth_lock.sequencer.reset();
                    sample_counter.fetch_sub(seq_samples, Ordering::Relaxed);
                }
            }
        },
        move |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    Ok(stream)
}

/// Gets the default audio output device.
fn get_audio_device() -> Result<Device, Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no output device available")?;
    Ok(device)
}

/// Retrieves the stream configuration for the given audio device.
fn get_stream_config(device: &Device) -> Result<StreamConfig, Box<dyn Error>> {
    // Retrieve the supported output configurations.
    let supported_configs = device.supported_output_configs()?;
    let supported_config = supported_configs
        .filter(|config| config.sample_format() == SampleFormat::F32)
        .next()
        .ok_or("no supported output configuration with f32 sample format")?;

    // Choose the configuration with the maximum sample rate.
    let config: StreamConfig = supported_config.with_max_sample_rate().config();

    Ok(config)
}
