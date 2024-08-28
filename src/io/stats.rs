use std::sync::mpsc::Receiver;

use crate::constants::STATS_TIMER_RESOLUTION_MS;
use crate::io::timer::Timer;

use crossterm::cursor;
use crossterm::style;
use crossterm::style::Color;
use crossterm::style::PrintStyledContent;
use crossterm::style::Stylize;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

use std::io::Stderr;
use std::io::Write;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> std::io::Result<()> {
    let mut total_bytes = 0;
    let start = std::time::Instant::now();
    let mut timer = Timer::new(STATS_TIMER_RESOLUTION_MS);
    let mut stderr = std::io::stderr();

    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();

        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs()._to_string(),
                rate_per_second,
            );
        }

        if num_bytes == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{bytes} bytes ")).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{rate:.0} b/s]")).with(Color::Blue);

    let _ = crossterm::execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
}

pub trait TimeOutput {
    fn _to_string(&self) -> String;
}

impl TimeOutput for u64 {
    fn _to_string(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}
