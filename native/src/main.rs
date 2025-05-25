use clap::{ArgGroup, Args, Parser, ValueEnum};
use std::path::PathBuf;

mod build;
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
    env_logger::init();
    let cli = Cli::parse();
    match () {
        _ if cli.build.is_some() => build::build(cli),
        _ => {
            wasm::run(
                cli.run_path
                    .as_ref()
                    .unwrap_or_else(|| cli.run.as_ref().unwrap()),
            )
            .unwrap();
            // TODO Only if display wanted?
            display::run();
        }
    }
}
