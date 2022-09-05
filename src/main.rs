use anyhow::Result;
use autoclap::autoclap;
use clap::{Arg, Command};
use pipeview::bar::DEFAULT_PIPEVIEW_SIZE;

#[tokio::main]
async fn main() -> Result<()> {
    let app: Command = autoclap!();
    let _ = app
        .arg(
            Arg::new("raw")
                .long("raw")
                .short('r')
                .help("Current, min, max stats from raw data as float."),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    let mut pipeview = pipeview::bar::WrappedBar::new(DEFAULT_PIPEVIEW_SIZE);

    loop {
        pipeview.update();
    }
}
