# reversi-rust

The game [reversi](https://en.wikipedia.org/wiki/Reversi), build in Rust.

## Installation

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
