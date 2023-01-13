use clap::{Parser, command};

mod subcommands;

#[derive(Parser)]
#[clap(version, author, about)]
struct Opts {
    #[command(subcommand)]
    subcommand: subcommands::Subcommands,
}
