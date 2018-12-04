extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("standup")
        .version("0.1")
        .author("David Nix <https://github.com/DavidNix>")
        .about("Remember your standup notes and post to slack.")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add note about something you did today")
                .alias("a")
                .arg(
                    Arg::with_name("NOTE")
                        .help("short note to add")
                        .required(true),
                ),
        ).get_matches();

    if let Some(add) = matches.subcommand_matches("add") {
        println!("GOT NOTE: {}", add.value_of("NOTE").unwrap());
    } else {
        println!("{}", matches.usage())
    }
}
