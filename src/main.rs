use anyhow::Result;
use autoclap::autoclap;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let app: clap::Command = autoclap!()
        .arg(
            Arg::new("input")
                .help("Raw data. TODO: Replace this with stdin")
                .required(true),
        )
        .arg(
            Arg::new("regex")
                .help("Regular expression groups to match the input.")
                .required(true),
        )
        .arg(
            Arg::new("colors")
                .help("Actual colors for the matched groups.")
                .required(true),
        );

    let args = app.clone().try_get_matches().unwrap_or_else(|e| e.exit());

    let input = args.get_one::<String>("input").unwrap();
    let regex = args.get_one::<String>("regex").unwrap();
    let colors = args.get_one::<String>("colors").unwrap();

    let _ = pipeview::colorizer::colorize(&input, &regex, &colors).unwrap();
    // let mut pipeview = pipeview::bar::WrappedBar::new(DEFAULT_PIPEVIEW_SIZE);
    // loop {
    //     pipeview.update();
    // }
    Ok(())
}
