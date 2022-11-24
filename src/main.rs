use anyhow::Result;
use autoclap::autoclap;
use clap::{Arg, Command};
use std::io::BufRead;

#[tokio::main]
async fn main() -> Result<()> {
    let app: clap::Command = autoclap!()
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

    let regex = args.get_one::<String>("regex").unwrap();
    let colors = args.get_one::<String>("colors").unwrap();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,    // with ^Z
            Ok(s) => {
                let _ = pipeview::colorizer::colorize(&s, &regex, &colors).unwrap();
                println!("");
            }
        }

    }

    // let mut pipeview = pipeview::bar::WrappedBar::new(DEFAULT_PIPEVIEW_SIZE);
    // loop {
    //     pipeview.update();
    // }
    Ok(())
}
