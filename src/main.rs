use anyhow::Result;
use taca::run;

fn main() -> Result<()> {
    pollster::block_on(run())
}
