# Hammster

Hammster is a Next.js web app plus a ZK circuit written in [halo2](https://halo2.dev/). It takes two 8-length vector inputs of binary digits and their [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) and generates a proof that the two inputs are the claimed hamming distance away from each other. 

# Prerequisites

- [Node.js & NPM](https://nodejs.org/en/download)
- [Rust](https://www.rust-lang.org/tools/install)
- [rustup](https://rustup.rs/)
- run `rustup toolchain install stable-aarch64-apple-darwin` (for users w/ Apple M processors)
- [wasm-pack](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#rust_environment_setup)

# Getting started

Install required dependencies:

```
yarn
```

Start the next.js server:

```
yarn dev
```

Build the wasm packages (you will need to remove `target = "aarch64-apple-darwin"` in `./circuits/.cargo/config` if not using an Apple M processor; I have not tried w/ other platforms):

```
yarn build:wasm
```
