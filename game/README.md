# Game

This uses a standard Rust setup.

Run `cargo build` or `cargo run` to build or run the game.

See [notes.md](notes.md) for resources I used to learn bevy. Unused code from the resources I used for reference can be found in `src/practise`.

Assets are made by me. Feel free to use them under CC-BY-SA 4.0 and link to this repo if you use them somewhere else.

## Raspberry Pi

You can compile the game for Raspberry Pi using [cross](https://github.com/cross-rs/cross). A Cross.toml is included. There is also a deploy script that you can use. It assumes ssh connection using and a pub key setup. You need to adjust the paths tho.