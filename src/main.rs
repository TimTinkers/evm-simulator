use clap::Parser;
use clap_verbosity_flag::Verbosity;

use evm_simulator::telemetry::{create_subscriber, init_subscriber};

// TODO: implement clap_complete with a proper installation via cargo.

/// A simulator of EVM bytecode.
#[derive(Parser)]
struct Cli {
  /**
    The path to a program file. A program file may be a file directly
    containing valid EVM bytecode or an intermediate language which can be
    compiled into EVM bytecode.
  */
  #[arg(short, long)]
  program_path: std::path::PathBuf,

  #[command(flatten)]
  verbose: Verbosity,
}

#[tracing::instrument(level = "debug")]
fn log_shit(number: u32) {}

fn main() {
  let cli = Cli::parse();
  let log_level = cli.verbose.log_level_filter();

  // Prepare tracing capabilities.
  let subscriber =
    create_subscriber("evm-simulator", log_level, std::io::stdout);
  init_subscriber(subscriber);

  // Log some bullshit.
  log_shit(100);
}
