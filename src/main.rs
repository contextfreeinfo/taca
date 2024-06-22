#[cfg(not(target_arch = "wasm32"))]
use clap::{Args, Parser, Subcommand};
#[cfg(not(target_arch = "wasm32"))]
use wasmic::run;

mod platform;
mod shaders;
mod wasmic;

#[cfg(not(target_arch = "wasm32"))]
#[derive(clap::Parser)]
#[command(about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Subcommand)]
enum Commands {
    Run(BuildArgs),
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Args)]
pub struct BuildArgs {
    pub app: String,
}

fn main() {
    // TODO Push all variation into wasmic?
    #[cfg(target_arch = "wasm32")]
    {
        // TODO Different loading for browser.
        wasmic::wasmish();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Run(args) => {
                run(args.app.clone());
            }
        }
    }
}
