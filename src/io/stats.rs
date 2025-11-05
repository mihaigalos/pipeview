use std::sync::mpsc::Receiver;

use crate::constants::STATS_TIMER_RESOLUTION_MS;
use crate::io::timer::Timer;
use crate::constants::*;
use crate::bar::WrappedBar;

use crossterm::cursor;
use crossterm::style;
use crossterm::style::Color;
use crossterm::style::PrintStyledContent;
use crossterm::style::Stylize;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

use std::io::Stderr;
use std::io::Write;

pub fn loop_stats(silent: bool, stats_rx: Receiver<usize>) -> std::io::Result<()> {
    loop_stats_with_size(silent, stats_rx, None)
}

pub fn loop_stats_with_size(silent: bool, stats_rx: Receiver<usize>, total_size: Option<u64>) -> std::io::Result<()> {
    let mut total_bytes = 0;
    let start = std::time::Instant::now();
    let mut timer = Timer::new(STATS_TIMER_RESOLUTION_MS);
    let mut stderr = std::io::stderr();
    
    let mut progress_bar = total_size.map(WrappedBar::new_for_transfer);

    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();

        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;
            
            if let Some(ref mut bar) = progress_bar {
                bar.update_transfer(total_bytes, rate_per_second, start.elapsed().as_secs());
            } else {
                output_progress(
                    &mut stderr,
                    total_bytes,
                    start.elapsed().as_secs()._to_string(),
                    rate_per_second,
                );
            }
        }

        if num_bytes == 0 {
            break;
        }
    }

    if let Some(ref mut bar) = progress_bar {
        bar.finish_transfer();
    } else if !silent {
        eprintln!();
    }

    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = bytes.as_human_readable("");
    let bytes = style::style(format!("{bytes} bytes ")).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = rate.as_human_readable("/s");
    let rate = style::style(format!(" {rate}")).with(Color::Blue);

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

pub trait BytesOutput {
    fn as_human_readable(&self, suffix: &str) -> String;
}

impl BytesOutput for f64 {
    fn as_human_readable(&self, suffix: &str) -> String {
        let (unit, description) = if *self > EXA.0 {
            EXA
        } else if *self >= PETA.0 && *self < EXA.0 {
            PETA
        } else if *self >= TERRA.0 && *self < PETA.0 {
            TERRA
        } else if *self >= GIGA.0 && *self < TERRA.0 {
            GIGA
        } else if *self >= MEGA.0 && *self < GIGA.0 {
            MEGA
        } else if *self >= KILO.0 && *self < MEGA.0 {
            KILO
        } else {
            BYTE
        };

        let result = *self / unit;

        format!("{result:.3}{description}{suffix}")
    }
}

impl BytesOutput for usize {
    fn as_human_readable(&self, suffix: &str) -> String {
        (*self as f64).as_human_readable(suffix)
    }
}
