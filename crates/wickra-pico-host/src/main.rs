//! The Wickra Pico host tool: generate the golden reference and check parity.

mod gen;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "wickra-pico-host",
    about = "Generate and verify the Wickra Pico golden signal sequence."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Regenerate the CSV feed, the embedded FEED const, and the expected signals.
    Bless,
    /// Check the recomputed signal sequence against the committed golden.
    Check,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Bless => {
            gen::bless();
            ExitCode::SUCCESS
        }
        Command::Check => gen::check(),
    }
}
