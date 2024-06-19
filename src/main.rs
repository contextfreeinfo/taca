#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::Read};

#[cfg(not(target_arch = "wasm32"))]
use clap::{Args, Parser, Subcommand};
#[cfg(not(target_arch = "wasm32"))]
use miniquad::*;

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

#[cfg(not(target_arch = "wasm32"))]
pub fn run(path: String) {
    let mut buf = Vec::<u8>::new();
    let mut conf = conf::Conf::default();
    conf.platform.apple_gfx_api = conf::AppleGfxApi::Metal;
    conf.platform.webgl_version = conf::WebGLVersion::WebGL2;
    conf.window_title = "Taca".into();
    miniquad::start(conf, move || {
        File::open(path)
            .expect("Bad open")
            .read_to_end(&mut buf)
            .expect("Bad read");
        Box::new(wasmic::wasmish(&buf))
    });
}
