use anyhow::Result;
use atty::Stream;
use autoclap::autoclap;
use clap::{Arg, ArgAction, Command, Parser};
use std::env;
use std::io::BufRead;

use std::thread;

use pipeview::formats::traits::{Formatter, FormatterFromToml};
use pipeview::args::Args;
use pipeview::io::write::loop_write;
use pipeview::io::stats::loop_stats_with_size;
use pipeview::io::read::loop_read;
use std::sync::mpsc;

fn io_main() -> std::io::Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;
    
    let total_size = if let Some(ref input_file) = infile {
        std::fs::metadata(input_file).ok().map(|m| m.len())
    } else {
        // Try to get size from stdin if it's a regular file (redirected)
        use std::os::unix::io::AsRawFd;
        unsafe {
            let mut stat: libc::stat = std::mem::zeroed();
            let fd = std::io::stdin().as_raw_fd();
            if libc::fstat(fd, &mut stat) == 0 && (stat.st_mode & libc::S_IFREG) != 0 {
                Some(stat.st_size as u64)
            } else {
                None
            }
        }
    };
    let silent = silent || !std::env::var("PIPEVIEW_SILENT").unwrap_or_default().is_empty();

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn({
        let infile = infile.clone();
        let stats_tx = stats_tx.clone();
        let write_tx = write_tx.clone();
        move || loop_read(infile, stats_tx, write_tx)
    });

    let stats_handle = thread::spawn({
        move || loop_stats_with_size(silent, stats_rx, total_size)
    });

    let write_handle = thread::spawn({
        let outfile = outfile.clone();
        move || loop_write(outfile, write_rx) // Directly pass write_rx
    });

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    let (_, is_stdout) = (atty::is(Stream::Stdin), atty::is(Stream::Stdout));
    if !is_stdout {
      return Ok(io_main()?);
    }

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
            Arg::new("config")
                .long("config")
                .help("Parse input as a config log with configuration from ~/.config/pipeview.toml OR the current folder with filename pipeview.toml, containing sample config:\n\
                 \n\
                 [foo]\n\
                 regex=\"^(.*?) (.*?) (.*?): (.*?) (.*)\"\n\
                 colors=\"red green blue red green\"\n\
                 [bar]\n\
                 regex=\"^(.*?) (.*?) (.*?): (.*?) (.*)\"\n\
                 colors=\"green red blue red green\"\n\
                 \n\
                 Call with --config=foo or --config=bar.")
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
    } else if let Some(config) = args.get_one::<String>("config") {
        let config_name: String = config.parse().unwrap();
        pipeview::formats::custom::Custom::get_config(&config_name)
    } else {
        let ids = args.ids().map(|id| id.as_str()).collect::<Vec<_>>();

        if ids.contains(&"regex") && ids.contains(&"colors") {
            (
                String::from(args.get_one::<String>("regex").unwrap()),
                String::from(args.get_one::<String>("colors").unwrap()),
            )
        } else {
            pipeview::formats::custom::Custom::get_config("")
        }
    };

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(s) => {
                let _ = pipeview::colorizer::run(&s, &regex, &colors).unwrap();
                println!();
            }
        }
    }

    Ok(())
}
