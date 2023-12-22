use clap::Parser;

mod commands;

pub fn run() {
    let args = commands::Cli::parse();
    match args.command {
        commands::CliCommand::Create(args) => {
            commands::actions::create(args);
        }
        commands::CliCommand::List => {
            commands::actions::list();
        }
        commands::CliCommand::Add(args) => {
            commands::actions::add(args);
        }
        commands::CliCommand::Remove(args) => {
            commands::actions::remove(args);
        }
    }
}