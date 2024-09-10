# ZiK-ZaK-Zoo JOLT

ZiK-ZaK-Zoo JOLT is a Tic-Tac-Toe-like game with a twist, implemented in Rust and utilizing the [JOLT](https://jolt.a16zcrypto.com/) framework for zero-knowledge proofs.

## Features

1. Play against a computer opponent
2. Random seed generation for game initialization
3. Zero-knowledge proof generation and verification of player wins

## How it works

1. The game uses a custom pseudo random number generator ([LCR](https://en.wikipedia.org/wiki/Linear_congruential_generator) with [MMIX](https://en.wikipedia.org/wiki/MMIX) parameters) seeded by the time between 2 user inputs.
2. Players take turns placing 'Z' (human) and 'K' (computer) on a 3x3 grid.
3. The game records the seed and player moves.
4. After a win, the game generates a zero-knowledge proof of the player's victory (taking seed + moves as inputs), and verifies it.

## Components

- `src/main.rs`: Main game logic and user interface (host)
- `guest/src/lib.rs`: Zero-knowledge proof generation and verification (guest)
- `core/src/lib.rs`: Core game mechanics and data structures

## Dependencies

- JOLT SDK
- Custom `zikzakzoo-core` library for game logic

To run the game, use `cargo run --release` in the main directory.
