# Metronome CLI

A powerful and flexible command-line metronome designed for musicians, offering advanced timing control, beat randomization, speed ramping, and interactive playback. 

## Features

### Basic Metronome Functionality

- Play a metronome at a specified BPM.
- Optionally provide your own sound file to play for the click.
- Adjust BPM while playing using interactive terminal controls.
  
### Advanced Timing Options

- Configure bars-on/bars-off cycles, where bars-off blocks mute metronome playback.
- Mute beats randomly by percentage.  
- Gradually increase and decrease tempo within a given range at a configurable rate.   

### Bonus Features (Planned Enhancements)

- Play a drone note or continuous chord in the background.  
- Load user-supplied sounds, with optional pitch analysis and adjustment for custom notes.  
- Play metronome beats as notes or chords with configurable note length.  
- Record your playing while metronome is on and playback your performance when complete.  
- Analyze and visualize timing accuracy.  

### Missing Features

Some features of a standard metronome have not been included.

- Time signatures
- Accented notes
- Subdivisions

There are plenty of great metronomes available that provide these features if you need them. However, this metronome is intended to help you develop a better internal sense of timing. We want you to be able to keep track of the bar yourself, without relying on strong/weak beats from the metronome.

We prefer to keep the core metronome simple, in favor of more complex features to improve your playing and sense of time.

## Installation

### Direct Download

You can download a precompiled binary from the [GitHub Releases](https://github.com/eldss/metronome-cli/releases) page.

1. Go to the [Releases](https://github.com/eldss/metronome-cli/releases) page.
2. Download the appropriate executable for your operating system.
3. Move the file to a directory in your system's `PATH` (e.g., `/usr/local/bin` on Linux/macOS or `C:\Program Files\metronome-cli\` on Windows).

### Using Cargo

To install the metronome CLI with Cargo, ensure you have [Rust](https://www.rust-lang.org/) installed and run:

```sh
cargo install metronome-cli
```

### Compile Locally

Alternatively, you can build from source:

```sh
git clone https://github.com/eldss/metronome-cli.git
cd metronome-cli
cargo build --release
```

## Usage

### Basic Metronome

Play at 120 BPM using the default click:

```sh
metronome --bpm 120
```

Play using a custom sound file:

```sh
metronome --bpm 100 --file custom-click.wav
```

**Limitations**

- BPM is a whole number ranging from 30 to 300.
- If proving a custom sound file, the length of the sample when played must fit between beats. If playback overlaps with the next beat, it will be cut off.
- When specifying a custom click, the `--file` argument must be provided.

### Beat Dropping

There are two ways to drop - meaning mute - beats during playback. Either drop a continuous length of notes in a regular pattern, or set a percentage defining the chance any given beat will be dropped.

These features can be used to improve your internal sense of timing.

Play 4 beats on, 2 beats off:

```sh
metronome --bpm 120 --drop-beats 4,2
```

If the number of beats on and off is equal, you can specify just one number.

For example:

```sh
metronome --bpm 120 --drop-beats 4
```

This is equivalent to:

```sh
metronome --bpm 120 --drop-beats 4,4
```

Drop 25% of beats randomly:

```sh
metronome --bpm 120 --drop-rate 25
```

Note that you provide the number of _beats_ not bars. The metronome does not have a set time signature, so a bar has no meaning. However, using `--drop-beats` you can effectively create 4/4 timing, where one bar is played and the next muted.

**Limitations**

- Dropped beats are whole numbers ranging from 1 to 32.
- Dropped rates are whole percentages ranging from 1% to 99%.

### BPM Ramp (Speed Up/Slow Down)

Gradually increase from 60 BPM to 200 BPM at a rate of 5 BPM per second:

```sh
metronome --bpm 60 --ramp 200 --change-rate 5
```

Gradually decrease from 300 BPM to 100 BPM at a rate of the default 1 BPM per second:

```sh
metronome --bpm 300 --ramp 100
```

When using a ramp, the metronome will automatically reverse the ramp direction once it reaches end values.

**Limitations**

- Ramp is a BPM value with the same limitations: a whole number from 30 to 300.
- Rate is the change in BPM per second, defined as a whole number from 1 to 15.
- Dropping beats is not supported while ramping tempo.

### Interactive BPM Adjustment

While the metronome is playing the terminal will display the current BPM. You can adjust the tempo interactively by pressing the up/down arrow keys to increase and decrease the tempo. The change will be reflected in the terminal.

Changing the tempo is disabled in ramp mode.

While playing
- Press ↑ / ↓ to increase or decrease BPM.
  - Disabled in ramp mode
- Press `q` to quit.

### Drones & Chords (Planned)

#### Drones

Play a continuous drone note on top of a basic metronome click (middle C here):

```sh
metronome --bpm 100 --drone C3
```

Tones are generated internally and cannot currently be changed. But we hope to add some customization options in the near future.

You can also play a chord as a drone (C minor here):

```sh
metronome --bpm 100 --drone C3,Eb3,G3
```

**Limitations**

- Tones can range from `A1` to `G6`, where the letter is the note name and the number is the octave number (`C3` is middle C). Sharps and flats are supported using `#` for sharp and `b` for flat, as in `C#4` and `Db4`.
- The number of tones must be between 1 and 4.

#### Tones in Time

Tones and chords can also be played in time, creating a harmonic metronome:

```sh
metronome --bpm 60 --harmonic --tones C3,Eb3,G3
```

This example plays the same chord for every beat, but you can also define a chord progression:

```sh
metronome \
--bpm 60 \
--harmonic \
--tones "Cmaj(C3 E3 G3),Gmaj(G3 B3 D4),Amin(A3 C4 E4),Fmaj(F3 A3 C4)" \
--progression Cmaj,Gmaj,Amin,Fmaj \
--beats-per 4,4,2,2
```

In this case, the `tones` argument defines four different chords with the labels Cmaj, Gmaj Amin and Fmaj. These same identifiers are used in the `progression` argument to define the chord progression used. The number of beats each chord should use is defined with the `--beats-per` argument. In this case, Cmaj and Gmaj are used for 4 beats each, then Amin and Fmaj are used for 2 beats each.

`beats-per` can also accept a single number that is used for each condition, so `--beats-per 4` is equivalent to `--beats-per 4,4,4,4` in the example above. 

Note that the chord identifiers can be any alphanumeric characters plus the special character `#`. For example: `a,b,c,d` or `1,2,3,4` are equally valid as those above so long as they match between `tones` and `progression`. This would look like

```sh
...
--tones "a(C3 E3 G3),b(G3 B3 D4),c(A3 C4 E4),d(F3 A3 C4)" \
--progression a,b,c,d \
...
```

or 

```sh
...
--tones "1(C3 E3 G3),2(G3 B3 D4),3(A3 C4 E4),4(F3 A3 C4)" \
--progression 1,2,3,4 \
...
```

**Limitations**

- Every tone defined in `tones` must be used in `progression`.
- There can be at most 10 chords in tones, with each chord having between 1 and 4 notes.
- If you use the same tone chord key multiple times, the last one defined will be the one used.
- If `beats-per` is not a single number, then the length of the `progression` and `beats-per` arguments must be equal.
- If `progression` is present, `beats-per` must also be present. Further, `tones` is expected to take the form of `<ID>(T T T T)` with commas between each definition. Internal tones must be separated by spaces in this case
- Tones can range from `Ab1` to `G#6`, where the letter is a capital and represents the note name. The number is the octave number (`C3` is middle C). Sharps and flats are supported using `#` for sharp and `b` for flat, as in `C#4` and `Db4`.
- The number of tones must be between 1 and 4 per chord.
- Numbers in `beats-per` are whole numbers between 1 and 12.

### Recording & Analysis (Planned)

Record and playback performance:

```sh
metronome --bpm 120 --record
```

Once the metronome is stopped, the program will provide options to playback the recording.

Analyze timing accuracy:

```sh
metronome --bpm 120 --analyze
```

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.


## License

This project is licensed under the Apache 2.0 License.

## Appendix

### Valid Invocations

#### Basic Metronome:
Play the metronome with a basic click.

```sh
metronome --bpm <bpm>
```

#### Custom Click Sound:
Basic metronome with a custom sound.

```sh
metronome --bpm <bpm> --file <path_to_sound_file>
```

#### Beat Dropping Pattern:
Play with <on> beats played and <off> beats muted. Given one number, it will be used for <on> and <off>

```sh
metronome --bpm <bpm> --drop-beats <on,off>
metronome --bpm <bpm> --drop-beats <on_and_off>
```

#### Random Beat Dropping:
Mute a specified percentage of beats randomly during playback.

```sh
metronome --bpm <bpm> --drop-rate <drop_rate>
```

#### BPM Ramp:
Ramp the bpm from a start to an end tempo and back again, at a given rate.

```sh
metronome --bpm <bpm> --ramp <target_bpm>
metronome --bpm <bpm> --ramp <target_bpm> --change-rate <rate>
```

#### Drone Tones:
Play a drone note(s) while the metronome is playing. Can play from 1 to 4 notes at a time.

Drone tones cannot be used with the harmonic metronome.

```sh
metronome --bpm <bpm> --drone <drone_tones>
```

#### Harmonic Click with Tones:
Play the given notes instead of a click. Can play from 1 to 4 notes at a time.

```sh
metronome --bpm <bpm> --harmonic --tones <tones>
```

#### Harmonic Click with Chord Progression:
Play the given chord progression instead of a click. Like playing tones instead of a click, but allows defining groups of tones, the order they are played, and the number of beats each plays for.

```sh
metronome --bpm <bpm> --harmonic --tones <tones> --progression <progression> --beats-per <beats_per>
```

#### Recording:
Record your practice while the metronome is playing and play it back when finished.

Recording can be used with all other options.

```sh
metronome --bpm <bpm> --record
```

#### Analysis:
Analyze your playing accuracy.

Analysis can be used with all other options.

```sh
metronome --bpm <bpm> --analyze
```

### Invalid combinations

Summary of Invalid Combinations
- `--drop-beats` and `--drop-rate` cannot be used together.
- `--drop-beats` or `--drop-rate` cannot be used with `--ramp`.
- `--tones`, `--progression`, or `--beats-per` cannot be used without specifying `--harmonic`.
- `--progression` requires `--beats-per`.
- `--progression` and `--beats-per` must have matching lengths or `--beats-per` must be a single number.
- `--progression` requires `--tones` to define tone combinations (chords) and the IDs in tones must match those in the progression. 
- `--progression` cannot contain tone IDs not defined in `--tones`.
- `--drone` and `--tones` cannot be used together.