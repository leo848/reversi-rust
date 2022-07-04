pub mod play;

use clap::{builder::PossibleValuesParser, value_parser, Arg, Command, ValueSource};

fn cli() -> Command<'static> {
    Command::new("reversi")
        .version("0.3.2")
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
                .value_parser(value_parser!(u8).range(1..=8)),
        )
        .arg(
            Arg::new("animation-speed")
            .help("The speed of the animation")
            .long_help("How long it takes to animate one flip. 'slow' corresponds to 0.8 seconds, 'medium' to 0.3 seconds and 'fast' to 0.1 seconds.")
            .short('s')
            .long("speed")
            .takes_value(true)
            .value_parser(PossibleValuesParser::new(vec![
                "slow",
                "medium",
                "fast",
            ]))
            .ignore_case(true)
            .default_value("medium")
            .conflicts_with("no-animation"),
        )
        .arg(
            Arg::new("no-animation")
            .help("Disable the animation")
            .long("no-animation")
            .short('A')
            .conflicts_with("animation-speed")
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
        eprintln!("Please specify either --player or --bot");
    }
}
