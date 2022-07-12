# reversi-rust

The game [reversi](https://en.wikipedia.org/wiki/Reversi), build in Rust.

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/leo848/reversi-rust/Rust)
![Clippy Workflow Status](https://img.shields.io/github/workflow/status/leo848/reversi-rust/rust-clippy%20analyze?label=clippy&logo=rust)
![Lines of code](https://img.shields.io/tokei/lines/github/leo848/reversi-rust)


## Installation

### Cargo install

```sh
cargo install reversi-game
```

### Build from source

```sh
git clone https://github.com/leo848/reversi-rust
cd reversi-rust && cargo install --path .
```

## Usage
```
USAGE:
	reversi [OPTIONS]

OPTIONS:
	-h, --help		Print help information
	-p, --player	Play against another player
	-b, --bot		Play against a minimax bot
	-d, --depth		Choose the depth / strength of the bot, implies --bot (default: 3)
```
