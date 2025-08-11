use clap::{Arg, Command, Error};

pub mod seed;
pub mod data;


fn main() {
    let matches = Command::new("gardenwatch")
    .version("0.1.0")
    .author("Andrew Kerr <apkerr@yahoo.com>")
    .about("Garden planning tool")
    .subcommand(
        Command::new("init")
        .about("Initialize GardenWatch database")
    )
    .subcommand(
        Command::new("new")
            .about("Create a new entry")
            .arg(Arg::new("item")
                .required(true)
            ),
    )
    .subcommand(
        Command::new("show")
        .about("Show database entries")
            .arg(Arg::new("item")
                .required(true)
            ),
    )
    .get_matches();

    let cmd: &str = matches.subcommand_name().unwrap_or_default();

    match cmd {
        "init" => {
            let result = init();
            if result.is_err() {
                panic!("Error creating database!");
            }
        }
        "new" => {
            let item = matches.subcommand_matches("new").unwrap().get_one::<String>("item").unwrap().as_str();
            match item {
                "seed" => seed::seed::create_new().unwrap(),
                _ => panic!("Could not create new {item}")
            }
        },
        "show" => {
            let item = matches.subcommand_matches("show").unwrap().get_one::<String>("item").unwrap().as_str();
            match item {
                "seed" => {
                    let seeds = seed::seed::get_all();
                    if seeds.is_err() {
                        panic!("Could not fetch all seeds");
                    }
                    println!("{:#?}", seeds.unwrap());
                },
                _ => println!("Could not create new {item}")
            }
        }
        _ => panic!("Unsupported command!")
    }


}

fn init() -> Result<(), native_db::db_type::Error> {
    println!("Initializing GardenWatch database");
    return data::data::init();
}
