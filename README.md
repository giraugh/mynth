# Mynth

Toy little modular synth in rust

## Building

Requires `ffmpeg`. On OSX you can install it with brew

```bash
brew install ffmpeg
```

Then use cargo to run and produce a raw audio file
```bash
cargo run # creates test_audio.bin
```

Then use ffmpeg's `ffplay` tool to play it
```bash
ffplay -autoexit -f f32le -ar 48000 -showmode 1 test_audio.bin
```
