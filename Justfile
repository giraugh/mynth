play:
  rm -f test_audio.bin
  cargo run
  ffplay -autoexit -f f32le -ar 48000 -showmode 1 test_audio.bin
