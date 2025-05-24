use clap::{ArgGroup, Args, Parser, ValueEnum};
use std::path::PathBuf;

mod display;
mod wasm;

#[derive(Parser)]
#[command(about, version, long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .args(&["run_path", "build", "run"])
        .multiple(false)
        .required(true)
))]
struct Cli {
    /// Run a Taca app (--run implied)
    #[arg(value_name = "run_path")]
    run_path: Option<PathBuf>,

    /// Build a Taca app from ready files
    #[arg(long, value_name = "PATH")]
    build: Option<PathBuf>,

    /// Run a Taca app (default action)
    #[arg(long, value_name = "PATH")]
    run: Option<PathBuf>,
}

fn main() {
    // wgpu uses `log` for all of our logging, so we initialize a logger with the `env_logger` crate.
    //
    // To change the log level, set the `RUST_LOG` environment variable. See the `env_logger`
    // documentation for more information.
    env_logger::init();
    let cli = Cli::parse();
    // TODO Build.
    assert!(cli.build.is_none());
    wasm::run(
        cli.run_path
            .as_ref()
            .unwrap_or_else(|| cli.run.as_ref().unwrap()),
    )
    .unwrap();
    display::run();
}
