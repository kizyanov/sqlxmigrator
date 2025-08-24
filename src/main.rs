use anyhow::Result;
use clap::{Arg, Command};
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("Migrations Manager")
        .version("0.1.0")
        .author("BizKey")
        .about("Database migration management tool")
        .subcommand(Command::new("run").about("Run all pending migrations"))
        .subcommand(Command::new("revert").about("Revert the last migration"))
        .subcommand(Command::new("status").about("Show migration status"))
        .subcommand(
            Command::new("create")
                .about("Create a new migration")
                .arg(Arg::new("name").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        _ => {
            println!("No command specified. Use --help for usage information.");
            process::exit(1);
        }
    }
}
