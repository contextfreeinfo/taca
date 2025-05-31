use anyhow::Result;
use clap::{ArgGroup, Parser};
use std::path::PathBuf;

mod archive;
mod build;
mod display;
mod storage;
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

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    // TODO Extract core features to `fly` and make `taca` specifically multimedia?
    match () {
        _ if cli.build.is_some() => build::build(cli),
        _ => {
            let path = cli
                .run_path
                .as_ref()
                .unwrap_or_else(|| cli.run.as_ref().unwrap());
            let archive = archive::Archive::from_path(path)?;
            let runtime = wasm::run(archive).unwrap();
            // TODO Open window only if wanted?
            display::run(runtime);
        }
    }
    Ok(())
}

// use std::thread;
// use tokio::runtime::Runtime;
// fn main() {
//     let io_thread = thread::spawn(|| {
//         let rt = Runtime::new().unwrap();
//         rt.block_on(async {
//             // async I/O loop
//             loop {
//                 // await something, e.g., socket.read()
//                 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//                 println!("I/O thread tick");
//             }
//         });
//     });
//     io_thread.join().unwrap();
// }
