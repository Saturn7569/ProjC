use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "projc")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        #[arg(help = "The name of the new project")]
        name: String,
    },

    Build,
    Clean,
}