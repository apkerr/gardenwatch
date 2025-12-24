pub mod cli {
    use std::path::PathBuf;

    use clap::{Args, Parser, Subcommand, ValueEnum};

    #[derive(Parser, Debug)]
    #[command(name = "gardenwatch")]
    #[command(about = "Garden planning tool", long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Debug, Subcommand)]
    pub enum Commands {
        // create a new item
        #[command(arg_required_else_help = true)]
        New(ItemArgs),
        // get an item
        #[command(arg_required_else_help = true)]
        Get(ItemArgs),
        // export to file
        #[command(arg_required_else_help = true)]
        Export(ImpExpArgs),
        // import from file
        #[command(arg_required_else_help = true)]
        Import(ImpExpArgs),
        // start web server
        #[command()]
        Start,
    }

    #[derive(Debug, Args)]
    #[command(flatten_help = true)]
    pub struct ItemArgs {
        #[command(subcommand)]
        pub sub_command: Option<ItemCommands>,
        #[arg(value_enum)]
        pub entry: Items,
    }

    #[derive(Debug, Subcommand)]
    pub enum ItemCommands {
        All,
        Id(ItemCommandArgs),
        Type(ItemCommandArgs),
    }

    #[derive(Debug, Args)]
    pub struct ItemCommandArgs {
        #[arg(required = true)]
        pub arg: String
    }


    #[derive(Debug, Clone)]
    pub enum Items {
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

    #[derive(Debug, Args)]
    #[command(flatten_help = true)]
    pub struct ImpExpArgs {
        pub f_name: PathBuf
    }


}

