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
    #[command(subcommand)]
    sub_command: Option<ItemCommands>,
    #[arg(value_enum)]
    entry: Items,
}

#[derive(Debug, Subcommand)]
enum ItemCommands {
    All,
    Id(ItemCommandArgs),
    Type(ItemCommandArgs),
}

#[derive(Debug, Args)]
struct ItemCommandArgs {
    #[arg(required = true)]
    arg: String
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    match args.command {
        Commands::Init => {
            let result = init();
            match result {
                Ok(_) => { 
                    println!("Database initialized"); 
                    Ok(()) 
                }
                Err(e) => panic!("{}", e)
            }
        }
        Commands::New(args) => {
            println!("New {:?}", args.entry);
            match args.entry {
                Items::Seed => {
                    let result = seed::seed::create_new();
                    let _ = result.inspect_err(|e| eprintln!("failed to create new {}", e));
                    Ok(())
                }
                _ => Ok(())
            }

        }
        Commands::Get(args) => {
            println!("Get {:?}", args.entry);
            match args.entry {
                Items::Seed => {
                    let seed_cmd = args.sub_command.unwrap_or(ItemCommands::All);
                    match seed_cmd {
                        ItemCommands::All => {
                            let seeds = seed::seed::get_all();
                            match seeds {
                                Ok(s) => {
                                    println!("{:#?}", s);
                                    Ok(())
                                }
                                Err(e) => panic!("{}", e)
                            }
                        }
                        ItemCommands::Id(cmd_arg) => {
                            let id = cmd_arg.arg.parse::<i32>()?;
                            let seed = seed::seed::get_by_id(id)?;
                            match seed {
                                None => { 
                                    println!("Could not find seed with id {id}");
                                    Ok(())
                                }
                                Some(s) => {
                                    println!("{:#?}", s);
                                    Ok(())
                                } 
                            }

                        }
                        ItemCommands::Type(cmd_arg) => {
                            let s_type = cmd_arg.arg;
                            let seeds = seed::seed::get_by_type(&s_type)?;
                            match seeds {
                                None => { 
                                    println!("Could not find seeds with type {}", &s_type);
                                    Ok(())
                                }
                                Some(s) => {
                                    println!("{:#?}", s);
                                    Ok(())
                                } 
                            }

                        }
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
