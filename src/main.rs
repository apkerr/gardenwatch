#[macro_use]
extern crate actix_web;

use core::panic;
use std::{env, io};

use clap::Parser;

mod seed;
use seed::{Seeds};
mod server;
mod cli;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load existing seeds
    let seeds = Seeds::load("test.seeds".into()).unwrap();

    unsafe { env::set_var("RUST_LOG", "actix_web=debug,actix_server=info") };
    env_logger::init();

    let args = cli::cli::Cli::parse();
    
    match args.command {
        cli::cli::Commands::New(args) => {
            println!("New {:?}", args.entry);
            match args.entry {
                cli::cli::Items::Seed => {
                    let result = seed::Seed::create_new();
                    let _ = result.inspect_err(|e| eprintln!("failed to create new {}", e));
                    Ok(())
                }
                _ => Ok(())
            }

        }
        cli::cli::Commands::Get(args) => {
            println!("Get {:?}", args.entry);
            match args.entry {
                cli::cli::Items::Seed => {
                    let seed_cmd = args.sub_command.unwrap_or(cli::cli::ItemCommands::All);
                    match seed_cmd {
                        cli::cli::ItemCommands::All => {
                            let seeds = seeds.get_all();
                            match seeds {
                                Ok(s) => {
                                    println!("{:#?}", s);
                                    Ok(())
                                }
                                Err(e) => panic!("{}", e)
                            }
                        }
                        cli::cli::ItemCommands::Id(cmd_arg) => {
                            let id = cmd_arg.arg.parse::<i32>();
                            let seed = seeds.get_by_id(id.unwrap());
                            todo!()

                        }
                        cli::cli::ItemCommands::Type(cmd_arg) => {
                            let s_type = cmd_arg.arg;
                            let seeds = seeds.get_by_type(&s_type);
                            todo!()

                        }
                    }

                }
            }
        }
        cli::cli::Commands::Import(arg) => {
            println!("Import {}", arg.f_name.to_str().unwrap());
            // seed::seed::import(arg.f_name)
            todo!()
        }
        cli::cli::Commands::Export(arg) => {
            let result = seeds.export(arg.f_name);
            Ok(())
        }
        cli::cli::Commands::Start => {
            let s = server::Server::init();
            s.start().await
        }
        
    }
}

