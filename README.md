# Boids

## About

This is Nishant Dash's implementation of 
<a href="https://www.red3d.com/cwr/boids/">Craig Reynold's Boids</a>.
It was created in rust using the Bevy game engine and was compiled to Web Assembly.
This was Nishant's first project using rust.

## What Are Boids?

Boids meant to simulate and model flocking behaviors like birds and fish. There are three simple behaviors that go into achieving this. 

1. Cohesion: Boids try to stay close to surrounding boids by moving the the average position of nearby boids.
1. Separation: Boids avoid crowding near other boids.
1. Alignment: Boids move in the average direction of neighboring boids.

Using these three behaviors, we are able to simulate flocking behavior.

## Build

If you have cargo install you can run:

```
cargo build
cargo run
```

To build for web run:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name boids --out-dir wasm --target web target/wasm32-unknown-unknown/release/boids.wasm
```