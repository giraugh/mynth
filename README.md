# Mynth

Toy little modular synth in rust

### Play a stream

`cargo run` will play the audio stream


### Play audio recording

To play a file created using `Recording::save` you will need `ffmpeg`.

On OSX you can install it with homebrew
```bash
brew install ffmpeg
```

Then use ffmpeg's `ffplay` tool to play it
```bash
ffplay -autoexit -f f32le -ar 48000 -showmode 1 my_recording.bin
```
