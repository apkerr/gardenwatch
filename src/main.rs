use core::panic;

use clap::{Args, Error, Parser, Subcommand, ValueEnum};
use serde::de::value::SeqAccessDeserializer;

pub mod seed;
pub mod data;
pub mod help;


#[derive(Parser, Debug)]
#[command(name = "gardenwatch")]
#[command(about = "Garden planning tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // initialize database
    Init,
    // create a new item
    #[command(arg_required_else_help = true)]
    New(ItemArgs),
    // get an item
    #[command(arg_required_else_help = true)]
    Get(ItemArgs),
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
struct ItemArgs {
    #[arg(value_enum)]
    entry: Items,
}

#[derive(Debug, Clone)]
enum Items {
    Seed,
}
impl ValueEnum for Items {
    fn value_variants<'a>() -> &'a [Self] {
        &[Items::Seed]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Items::Seed => Some("seed".into())
        }
    }
    
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        Self::value_variants()
            .iter()
            .find(|v| {
                v.to_possible_value()
                    .expect("ValueEnum::value_variants contains only values with a corresponding ValueEnum::to_possible_value")
                    .matches(input, ignore_case)
            })
            .cloned()
            .ok_or_else(|| std::format!("invalid variant: {input}"))
    }
}


fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Init => {
            let result = init();
            match result {
                Ok(_) => println!("Database initialized"),
                Err(e) => panic!("{}", e)
            }
        }
        Commands::New(args) => {
            println!("New {:?}", args.entry);
            match args.entry {
                Items::Seed => {
                    let result = seed::seed::create_new();
                    let _ = result.inspect_err(|e| eprintln!("failed to create new {}", e));
                }
            }

        }
        Commands::Get(args) => {
            println!("Get {:?}", args.entry);
            match args.entry {
                Items::Seed => {
                    let seeds = seed::seed::get_all();
                    match seeds {
                        Ok(s) => {
                            println!("{:#?}", s)
                        }
                        Err(e) => panic!("{}", e)
                    }
                }
            }
        }
    }
}

fn init() -> Result<(), native_db::db_type::Error> {
    println!("Initializing GardenWatch database");
    return data::data::init();
}
