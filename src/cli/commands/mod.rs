pub mod actions;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand
}


#[derive(Subcommand)]
pub enum CliCommand {
    Create(CreateArgs),
    List,
    Add(AddArgs),
    Remove(RemoveArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(short, long)]
    pub name: String,
}

#[derive(Args)]
pub struct AddArgs {
    #[arg(short, long)]
    pub list: String,

    #[arg(short, long)]
    pub title: String,
}


#[derive(Args)]
pub struct RemoveArgs {
     #[arg(short, long)]
    pub list: String,

    #[arg(short, long)]
    pub id: i32,
}