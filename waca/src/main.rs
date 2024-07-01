use clap::{Args, Parser, Subcommand};
use winit::event_loop::EventLoop;

mod app;
mod display;
use crate::app::Wrap;
use crate::display::Display;

#[derive(Parser)]
#[command(about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(BuildArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    pub app: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(args) => {
            run(args.app.clone());
        }
    }
}

fn run(app: String) {
    // Setup based on: https://github.com/erer1243/wgpu-0.20-winit-0.30-web-example
    let app = Wrap::load(&app);
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut display = Display::new(&event_loop, app);
    event_loop.run_app(&mut display).unwrap();
}
