/// Regex pattern to match chord input strings.
/// - `[A-Za-z0-9_+\-#]{1,10}` Matches 1 to 10 characters for the ID. The allowed characters are letters (case-insensitive), digits, underscore, plus, minus, and hash.
/// - `\(` Matches the literal opening parenthesis.
/// - `\s*` Allows optional whitespace right after the opening parenthesis.
/// - `[^)]+` Matches one or more characters that are not a closing parenthesis—this ensures there's something inside the parentheses.
/// - `\s*` Allows optional whitespace before the closing parenthesis.
/// - `\)` Matches the literal closing parenthesis.
///
/// Doesn't validate the contents inside the parentheses, but simply ensures that there is at least one non-) character between them.
pub const CHORD_REGEX: &str = r"[A-Za-z0-9_+\-#]{1,10}\(\s*[^)]+\s*\)";

/// Regex for individual note values.
/// - `[A-G]` Matches a note letter (A–G).
/// - `(?:[#b])?` Optionally matches an accidental (# or b).
/// - `[2-5]` Matches an octave digit from 2 to 5.
pub const NOTE_REGEX: &str = r"[A-G](?:[#b])?[2-5]";

/// Precomputed list of note names and their frequencies from Cb2 to B#5.
pub const NOTE_FREQUENCIES: [(&str, f32); 84] = [
    ("Cb2", 61.74),
    ("C2", 65.41),
    ("C#2", 69.30),
    ("Db2", 69.30),
    ("D2", 73.42),
    ("D#2", 77.78),
    ("Eb2", 77.78),
    ("E2", 82.41),
    ("Fb2", 82.41),
    ("E#2", 87.31),
    ("F2", 87.31),
    ("F#2", 92.50),
    ("Gb2", 92.50),
    ("G2", 98.00),
    ("G#2", 103.83),
    ("Ab2", 103.83),
    ("A2", 110.00),
    ("A#2", 116.54),
    ("Bb2", 116.54),
    ("B2", 123.47),
    ("Cb3", 123.47),
    ("B#2", 130.81),
    ("C3", 130.81),
    ("C#3", 138.59),
    ("Db3", 138.59),
    ("D3", 146.83),
    ("D#3", 155.56),
    ("Eb3", 155.56),
    ("E3", 164.81),
    ("Fb3", 164.81),
    ("E#3", 174.61),
    ("F3", 174.61),
    ("F#3", 185.00),
    ("Gb3", 185.00),
    ("G3", 196.00),
    ("G#3", 207.65),
    ("Ab3", 207.65),
    ("A3", 220.00),
    ("A#3", 233.08),
    ("Bb3", 233.08),
    ("B3", 246.94),
    ("Cb4", 246.94),
    ("B#3", 261.63),
    ("C4", 261.63),
    ("C#4", 277.18),
    ("Db4", 277.18),
    ("D4", 293.66),
    ("D#4", 311.13),
    ("Eb4", 311.13),
    ("E4", 329.63),
    ("Fb4", 329.63),
    ("E#4", 349.23),
    ("F4", 349.23),
    ("F#4", 369.99),
    ("Gb4", 369.99),
    ("G4", 392.00),
    ("G#4", 415.30),
    ("Ab4", 415.30),
    ("A4", 440.00),
    ("A#4", 466.16),
    ("Bb4", 466.16),
    ("B4", 493.88),
    ("Cb5", 493.88),
    ("B#4", 523.25),
    ("C5", 523.25),
    ("C#5", 554.37),
    ("Db5", 554.37),
    ("D5", 587.33),
    ("D#5", 622.25),
    ("Eb5", 622.25),
    ("E5", 659.25),
    ("Fb5", 659.25),
    ("E#5", 698.46),
    ("F5", 698.46),
    ("F#5", 739.99),
    ("Gb5", 739.99),
    ("G5", 783.99),
    ("G#5", 830.61),
    ("Ab5", 830.61),
    ("A5", 880.00),
    ("A#5", 932.33),
    ("Bb5", 932.33),
    ("B5", 987.77),
    ("B#5", 1046.50),
];
