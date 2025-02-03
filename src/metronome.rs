use crate::audio::{self, AudioOutput};
use crate::config::AppConfig;
use crate::scheduler::Scheduler;
use crate::synth::Synth;

pub struct Metronome {
    pub config: AppConfig,
    scheduler: Scheduler,
    // Abstract the audio output so we can swap implementations later.
    audio_output: Box<dyn AudioOutput>,
    synth: Synth,
}

impl Metronome {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            scheduler: Scheduler::new(),
            audio_output: Box::new(audio::DefaultAudioOutput::new()),
            synth: Synth::new(),
        }
    }

    pub fn run(&mut self) {
        // Main loop: schedule the next tone playback based on current BPM,
        // handle beat dropping and ramping if enabled.
        // Use the scheduler to get timing events, then call audio_output.play(synth.generate_tone(...)).
        todo!("Implement main metronome run loop")
    }

    pub fn adjust_bpm(&mut self, new_bpm: u32) {
        todo!("Adjust BPM and notify scheduler")
    }
}
