use clap::Subcommand;

mod daily;
mod make_neuron;
mod setup;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Triggers the daily job to: apply interest, flush pending deposits, split new withdrawal
    /// neurons.
    Daily(daily::Command),
    /// Make a new neuron owned by the signing canister
    MakeNeuron(make_neuron::Command),
    /// Initial setup job, to: create the new neurons and add them.
    Setup(setup::Command),
}
