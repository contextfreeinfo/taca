use clap::{Args, Parser, Subcommand};
use display::DisplayOptions;
use winit::event_loop::EventLoop;

mod app;
mod display;
mod gpu;
mod key;
mod sound;
mod text;
mod wasi;
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
    Run(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    pub app: String,
    #[arg(long, num_args = 2, value_names = ["SIZE_X", "SIZE_Y"])]
    pub size: Vec<f64>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(args) => {
            run(args);
        }
    }
}

fn run(args: &RunArgs) {
    // Setup based on: https://github.com/erer1243/wgpu-0.20-winit-0.30-web-example
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let options = DisplayOptions {
        size: match args.size.as_slice() {
            &[size_x, size_y] => Some((size_x, size_y)),
            _ => None,
        },
    };
    let display = Display::new(&event_loop, options);
    let mut app = Box::new(App::load(&args.app, display));
    // This should be safe because we only initiate app activity from display itself.
    // TODO Ensure that all event handling is on a single thread.
    let ptr = &mut *app as *mut App;
    app.run(event_loop, ptr);
}
