# Changelog

All notable changes to the Metronome CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-04-12

### Added
- First official release on crates.io
- Added crates.io badge to README

## [0.3.0] - 2025-03-05

### Added
- Chord progression feature
  - Support for defining multiple chords with custom identifiers
  - Configurable progression sequence
  - Customizable beats per chord in the progression
- Improved sound synthesis for hi-hat and electric piano

## [0.2.0] - 2025-03-02

### Added
- Random beat dropping feature with `--drop-rate` option
  - Configurable percentage (1-99%) for randomly muting beats
  - Input validation for drop rate range
- Improved documentation for beat dropping features

### Changed
- Adjusted amplitude constants for hi-hat and electric piano synthesis
- Updated installation instructions for Mac and Linux users

## [0.1.0] - 2025-02-20

### Added
- Initial release
- Basic metronome functionality with configurable BPM
- Beat muting with configurable bars-on/bars-off cycles
- Drone notes and chords
- Harmonic metronome with configurable tones

### Changed
- Multiple patch releases (0.1.1 - 0.1.11) with improvements to:
  - GitHub Actions workflow for automated releases
  - Build configurations
  - Documentation updates
