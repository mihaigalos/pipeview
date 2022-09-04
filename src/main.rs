use anyhow::Result;
use autoclap::autoclap;
use clap::{Arg, Command};
use pipeview::bar::DEFAULT_PIPEVIEW_SIZE;

#[tokio::main]
async fn main() -> Result<()> {
    let _: Command = autoclap!();
    let mut pipeview = pipeview::bar::WrappedBar::new(DEFAULT_PIPEVIEW_SIZE);

    loop {
        pipeview.update();
    }
}
