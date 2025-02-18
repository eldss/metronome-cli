use std::f64::consts::TAU;

/// A single voice (note) in the polyphonic synthesizer.
pub struct Voice {
    frequency: f64,
    phase: f64,
    elapsed: usize,  // sample counter since note-on
    duration: usize, // total duration in samples
    attack: usize,   // attack duration in samples
    release: usize,  // release duration in samples
}

impl Voice {
    /// Create a new voice.
    ///
    /// - `frequency`: frequency in Hz.
    /// - `duration`: total note duration in seconds.
    /// - `attack`: attack duration in seconds.
    /// - `release`: release duration in seconds.
    /// - `sample_rate`: the sample rate (Hz).
    pub fn new(frequency: f64, duration: f64, attack: f64, release: f64, sample_rate: f64) -> Self {
        Voice {
            frequency,
            phase: 0.0,
            elapsed: 0,
            duration: (duration * sample_rate) as usize,
            attack: (attack * sample_rate) as usize,
            release: (release * sample_rate) as usize,
        }
    }

    /// Returns true if the note has finished playing.
    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }

    /// Computes and returns the next audio sample.
    pub fn next_sample(&mut self, sample_rate: f64) -> f32 {
        // Compute a simple linear envelope:
        //   - During the attack, amplitude ramps from 0 to 1.
        //   - Then it holds at 1 until the release phase.
        //   - During release, amplitude decays linearly to 0.
        let env = if self.elapsed < self.attack {
            self.elapsed as f64 / self.attack as f64
        } else if self.elapsed > self.duration.saturating_sub(self.release) {
            let release_elapsed = self.elapsed - (self.duration - self.release);
            1.0 - (release_elapsed as f64 / self.release as f64)
        } else {
            1.0
        };

        // Generate a sine wave sample.
        let sample = (self.phase).sin() * env;
        self.phase += TAU * self.frequency / sample_rate;
        if self.phase > TAU {
            self.phase -= TAU;
        }
        self.elapsed += 1;
        sample as f32
    }
}

/// A simple polyphonic synthesizer that can play multiple notes concurrently.
pub struct PolySynth {
    voices: Vec<Voice>,
    sample_rate: f64,
}

impl PolySynth {
    /// Creates a new PolySynth.
    pub fn new(sample_rate: f64) -> Self {
        PolySynth {
            voices: Vec::new(),
            sample_rate,
        }
    }

    /// Triggers a new note.
    ///
    /// - `frequency`: in Hz.
    /// - `duration`: in seconds.
    /// - `attack`: in seconds.
    /// - `release`: in seconds.
    pub fn note_on(&mut self, frequency: f64, duration: f64, attack: f64, release: f64) {
        let voice = Voice::new(frequency, duration, attack, release, self.sample_rate);
        self.voices.push(voice);
    }

    /// Generates the next sample by summing all active voices.
    pub fn next_sample(&mut self) -> f32 {
        let mut sum = 0.0;
        for voice in self.voices.iter_mut() {
            sum += voice.next_sample(self.sample_rate);
        }

        // Remove voices that have finished playing.
        self.voices.retain(|v| !v.is_finished());
        sum
    }
}
