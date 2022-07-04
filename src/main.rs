pub mod play;

use clap::{value_parser, Arg, Command, ValueSource};

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
        .arg(
            Arg::new("depth")
                .help("The depth of the bot's search (implies --bot)")
                .short('d')
                .long("depth")
                .takes_value(true)
                .default_value("3")
                .value_parser(value_parser!(u8).range(0..=8)),
        )
}

fn main() {
    let matches = cli().get_matches();
    if matches.is_present("player") {
        play::run(&play::Opponent::Human, &matches);
    } else if matches.is_present("bot")
        || matches.value_source("depth").unwrap() != ValueSource::DefaultValue
    {
        play::run(&play::Opponent::Bot, &matches);
    } else {
        eprintln!("Please specify either player or bot.");
    }
}
