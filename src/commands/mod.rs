use clap::Subcommand;

mod daily;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Triggers the daily job to: apply interest, flush pending deposits, split new withdrawal
    /// neurons.
    Daily(daily::Command),
}
