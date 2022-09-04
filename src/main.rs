use anyhow::Result;
use autoclap::autoclap;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let _: Command = autoclap!();
    let mut vumeter = pipeview::bar::WrappedBar::new(100);

    loop {
        vumeter.update();
    }
}
