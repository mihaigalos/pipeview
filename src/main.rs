use anyhow::Result;
use autoclap::autoclap;
use clap::{Arg, ArgAction, Command};
use std::io::BufRead;

use pipeview::formats::traits::Formatter;

#[tokio::main]
async fn main() -> Result<()> {
    let app: clap::Command = autoclap!()
        .arg(
            Arg::new("regex")
                .help("Regular expression groups to match the input.")
                .required(false),
        )
        .arg(
            Arg::new("colors")
                .help("Actual colors for the matched groups.")
                .required(false),
        )
        .arg(
            Arg::new("nginx")
                .long("nginx")
                .help("Parse input as Nginx log.")
                .action(ArgAction::SetTrue)
                .required(false),
        );

    let args = app.clone().try_get_matches().unwrap_or_else(|e| e.exit());

    let (regex, colors) = match args.get_flag("nginx") {
        false => (
            args.get_one::<String>("regex").map(|s| s.as_str()).unwrap(),
            args.get_one::<String>("colors")
                .map(|s| s.as_str())
                .unwrap(),
        ),
        true => pipeview::formats::nginx::Nginx::get_config(),
    };

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break, // with ^Z
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
