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
            Arg::new("aim")
                .long("aim")
                .help("Parse input as mihaigalos/aim log.")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("nginx")
                .long("nginx")
                .help("Parse input as Nginx log.")
                .action(ArgAction::SetTrue)
                .required(false),
        );

    let args = app.try_get_matches().unwrap_or_else(|e| e.exit());

    let (regex, colors) = if args.get_flag("aim") {
        pipeview::formats::aim::Aim::get_config()
    } else if args.get_flag("nginx") {
        pipeview::formats::nginx::Nginx::get_config()
    } else {
        (args.get_one::<String>("regex").map(|s| s.as_str()).unwrap(),
        args.get_one::<String>("colors")
            .map(|s| s.as_str())
            .unwrap())
    };

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(s) => {
                let _ = pipeview::colorizer::colorize(&s, regex, colors).unwrap();
                println!();
            }
        }
    }

    // let mut pipeview = pipeview::bar::WrappedBar::new(DEFAULT_PIPEVIEW_SIZE);
    // loop {
    //     pipeview.update();
    // }
    Ok(())
}
