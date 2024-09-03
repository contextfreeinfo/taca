use clap::{Args, Parser, Subcommand};
use winit::event_loop::EventLoop;

mod app;
mod display;
mod gpu;
mod key;
mod text;
use crate::app::App;
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
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let display = Display::new(&event_loop);
    let mut app = Box::new(App::load(&app, display));
    // This should be safe because we only initiate app activity from display itself.
    // TODO Ensure that all event handling is on a single thread.
    let ptr = &mut *app as *mut App;
    app.run(event_loop, ptr);
}
