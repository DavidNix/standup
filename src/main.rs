extern crate clap;
extern crate dirs;
extern crate chrono;

use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs;
use chrono::prelude::*;


const FINISH_COMMAND: &str = "finish";

fn main() {
    let matches = App::new("standup")
        .version("0.1")
        .author("David Nix <https://github.com/DavidNix>")
        .about("Remember your standup notes and post to slack.")
        .subcommand(
            SubCommand::with_name(FINISH_COMMAND)
                .about("Add note about something you did today")
                .alias("fin")
                .arg(
                    Arg::with_name("TASK")
                        .help("short description of task you finished today")
                        .required(true),
                ),
        ).get_matches();

    if let Some(add) = matches.subcommand_matches(FINISH_COMMAND) {
        println!("GOT NOTE: {}", add.value_of("NOTE").unwrap());
    } else {
        println!("{}", matches.usage())
    }
}


// dialoguer
// https://github.com/mitsuhiko/dialoguer/blob/master/examples/checkboxes.rs

struct Cache {
    data: HashMap<u32, [String]>
}

impl Cache {
    fn create_cache() {
        let cache_dir = dirs::cache_dir()?.push("Standup/.cache")?;
        // https://doc.rust-lang.org/std/fs/struct.File.html#method.create
//        let day = Utc::now().day(); // use day as map key
    }
}
