pub struct Synth {
    // Internal settings for wave type, volume, etc.
}

impl Synth {
    pub fn new() -> Self {
        Self {
            // internal state
        }
    }

    /// Generate a tone based on current settings and possibly user-defined sound files.
    pub fn generate_tone(&self, tone_id: &str) -> Vec<u8> {
        // Returns a raw audio buffer.
        todo!("Generate tone data for the given tone identifier")
    }

    /// Generate a chord from multiple tones
    pub fn generate_chord(&self, tones: &[&str]) -> Vec<u8> {
        todo!("Mix multiple tones into a chord audio buffer")
    }
}
