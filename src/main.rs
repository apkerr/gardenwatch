use clap::{Arg, Command, Error};
// use crate::seed::seed;

pub mod seed;


fn main() {
    let matches = Command::new("gardenwatch")
    .version("0.1.0")
    .author("Andrew Kerr <apkerr@yahoo.com>")
    .about("Garden planning tool")
    .subcommand(
        Command::new("new")
            .about("Create a new entry")
            .arg(Arg::new("item")
                .required(true)
            ),
    )
    .get_matches();

    let cmd: &str = matches.subcommand_name().unwrap_or_default();

    match cmd {
        "new" => {
            let item = matches.subcommand_matches("new").unwrap().get_one::<String>("item").unwrap().as_str();
            match item {
                "seed" => seed::seed::create_new(),
                _ => println!("Could not create new {item}")
            }
        }
        _ => panic!("Unsupported command!")
    }


}

