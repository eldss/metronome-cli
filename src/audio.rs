use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, SampleFormat, Stream, StreamConfig,
};
use fundsp::prelude::*;
use rand::Rng;
use std::{
    error::Error,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use crate::{config::AppConfig, synth};

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
    app_config: &AppConfig,
) -> Result<Stream, Box<dyn Error>> {
    let device = get_audio_device()?;
    let stream_config = get_stream_config(&device)?;

    // Extract the sample rate as a f64 for calculations and build the output stream
    let sample_rate = stream_config.sample_rate.0 as f64;

    // Ensure we capture the correct number of beats in a loop
    let beats_per_sequence = if let Some((on, off)) = app_config.drop_beats {
        on + off
    } else if let Some(beats) = &app_config.beats_per {
        beats.iter().sum()
    } else {
        1
    };

    // Ensure we can drop beats during playback if given
    let drop_rate = if let Some(rate) = app_config.drop_rate {
        rate as f64 / 100.0
    } else {
        0.0
    };

    let stream = device.build_output_stream(
        &stream_config,
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
            let seq_samples =
                (beat_period * sample_rate * beats_per_sequence as f64).round() as u64;

            // Enable random beat drops
            let mut rng = rand::rng();

            // Process each frame in the output buffer.
            for frame in data.chunks_mut(stream_config.channels as usize) {
                // Retrieve the next sample from the sequencer.
                let sample = synth_lock.sequencer.get_mono();
                for sample_out in frame.iter_mut() {
                    *sample_out = sample as f32;
                }

                // Update the sample counter and reset the sequencer if a beat has completed.
                let prev_count = sample_counter.fetch_add(1, Ordering::Relaxed) + 1;
                if prev_count >= seq_samples {
                    // Given rate is chance of `true`
                    if rng.random_bool(1.0 - drop_rate) {
                        synth_lock.sequencer.reset();
                    }
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
    let mut supported_configs = device.supported_output_configs()?;
    let supported_config = supported_configs
        .find(|config| config.sample_format() == SampleFormat::F32)
        .ok_or("no supported output configuration with f32 sample format")?;

    // Choose the configuration with the maximum sample rate.
    let config: StreamConfig = supported_config.with_max_sample_rate().config();

    Ok(config)
}
