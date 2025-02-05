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
/// - `[1-6]` Matches an octave digit from 1 to 6.
pub const NOTE_REGEX: &str = r"[A-G](?:[#b])?[1-6]";
