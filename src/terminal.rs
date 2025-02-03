use crate::config::AppConfig;
use std::thread;

/// Spawns a dedicated thread to listen for terminal input.
pub fn spawn_terminal_handler(config: AppConfig) {
    thread::spawn(move || {
        // Setup terminal input (using crossterm or a similar library)
        loop {
            // Poll for input events (arrow keys for BPM adjustment, 'q' to quit, etc.)
            // When an event occurs, communicate it back to the core metronome logic,
            // perhaps via a channel or a callback.
            todo!("Implement terminal input handling and event dispatch")
        }
    });
}
