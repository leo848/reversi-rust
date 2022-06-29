pub mod play;

use clap::{Arg, Command};

fn cli() -> Command<'static> {
    Command::new("reversi")
        .version("0.1.0")
        .author("Leo Blume <leoblume@gmx.de>")
        .about("Play the Reversi game against another player or the computer.")
        .arg(
            Arg::new("player")
                .help("Play against another player")
                .short('p')
                .long("player")
                .conflicts_with("bot"),
        )
        .arg(
            Arg::new("bot")
                .help("Play against a bot")
                .short('b')
                .long("bot")
                .conflicts_with("player"),
        )
}

fn main() {
    let matches = cli().get_matches();
    if matches.is_present("player") {
        play::run(play::Opponent::Human, &matches);
    } else if matches.is_present("bot") {
        play::run(play::Opponent::Bot, &matches);
    } else {
        eprintln!("Please specify either player or bot.");
    }
}
