use clap::Parser;

mod commands;
mod deposits;
mod governance;
mod identity;
mod ledger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        commands::Command::Daily(c) => c.run().await?,
        commands::Command::Setup(c) => c.run().await?,
    }
    Ok(())
}
