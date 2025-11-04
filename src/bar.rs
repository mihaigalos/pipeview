use colored::Colorize;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use strfmt::strfmt;

const DEFAULT_PIPEVIEW_PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏  ";
const DEFAULT_PIPEVIEW_TEMPLATE: &str = "{spinner:.cyan}▕{bar:.white}▏{msg}";
const DEFAULT_PIPEVIEW_TICK: u64 = 100;
pub const DEFAULT_PIPEVIEW_SIZE: u64 = 100;

fn construct_progress_bar(
    total_size: u64,
    progress_chars: &str,
    template: &str,
    tick: u32,
) -> indicatif::ProgressBar {
    let pb = ProgressBar::new(total_size);

    pb.enable_steady_tick(Duration::new(0, tick));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .unwrap()
            .progress_chars(progress_chars),
    );
    pb
}

pub struct WrappedBar {
    pub output: indicatif::ProgressBar,
    pub max: f64,
    pub min: f64,
    pub total_size: u64,
}

impl WrappedBar {
    pub fn new(total_size: u64) -> WrappedBar {
        dotenv().ok();
        let progress_chars = &env::var("PIPEVIEW_PROGRESSBAR_PROGRESS_CHARS")
            .unwrap_or_else(|_| DEFAULT_PIPEVIEW_PROGRESS_CHARS.to_string());
        let template = &env::var("PIPEVIEW_PROGRESSBAR_TEMPLATE")
            .unwrap_or_else(|_| DEFAULT_PIPEVIEW_TEMPLATE.to_string());
        let tick = env::var("PIPEVIEW_PROGRESSBAR_TICK")
            .unwrap_or_else(|_| DEFAULT_PIPEVIEW_TICK.to_string())
            .parse::<u32>()
            .unwrap();
        let output = construct_progress_bar(total_size, progress_chars, template, tick);
        WrappedBar {
            output,
            max: 0.0,
            min: f64::MAX,
            total_size: total_size,
        }
    }
    pub fn update(&mut self) {
        let pos = self.compute_position();

        self.set_minmax(pos);
        self.output.set_position(pos as u64);
        self.output.set_length(self.max as u64);
        self.set_message(pos);
    }

    fn compute_position(&mut self) -> f64 {
        let previous_pos = self.output.position();

        std::io::stdin()
            .lines()
            .next()
            .unwrap_or_else(|| Ok(previous_pos.to_string()))
            .unwrap()
            .parse::<f64>()
            .unwrap_or(previous_pos as f64)
    }

    fn set_minmax(&mut self, pos: f64) {
        if self.max < pos {
            self.max = pos;
        }

        if self.min >= pos {
            self.min = pos;
        }
    }

    fn set_message(&mut self, pos: f64) {
        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("pos".to_string(), pos.to_string());
        vars.insert("min".to_string(), self.min.to_string());
        vars.insert("max".to_string(), self.max.to_string());
        self.output.set_message(
            strfmt(
                &format!(
                    "{}{}{}{}{}{}",
                    "Current:".to_string().green().bold(),
                    " {pos} ".to_string().white(),
                    "Min:".green().bold(),
                    " {min} ".to_string().white(),
                    "Max:".green().bold(),
                    " {max} ".to_string().white(),
                ),
                &vars,
            )
            .unwrap(),
        );
    }

    pub fn new_for_transfer(total_size: u64) -> WrappedBar {
        dotenv().ok();
        let progress_chars = &env::var("PIPEVIEW_PROGRESSBAR_PROGRESS_CHARS")
            .unwrap_or_else(|_| DEFAULT_PIPEVIEW_PROGRESS_CHARS.to_string());
        let template = &env::var("PIPEVIEW_PROGRESSBAR_TEMPLATE")
            .unwrap_or_else(|_| DEFAULT_PIPEVIEW_TEMPLATE.to_string());
        let tick = env::var("PIPEVIEW_PROGRESSBAR_TICK")
            .unwrap_or_else(|_| DEFAULT_PIPEVIEW_TICK.to_string())
            .parse::<u32>()
            .unwrap();
        let output = construct_progress_bar(total_size, progress_chars, template, tick);
        
        WrappedBar {
            output,
            max: total_size as f64,
            min: 0.0,
            total_size,
        }
    }

    pub fn update_transfer(&mut self, bytes_transferred: usize, rate: f64, elapsed_secs: u64) {
        self.output.set_position(bytes_transferred as u64);
        self.set_transfer_message(bytes_transferred, rate, elapsed_secs);
    }

    pub fn finish_transfer(&self) {
        self.output.finish_and_clear();
    }

    fn set_transfer_message(&mut self, bytes_transferred: usize, rate: f64, elapsed_secs: u64) {
        use crate::io::stats::{BytesOutput, TimeOutput};
        
        let transferred_str = (bytes_transferred as f64).as_human_readable("");
        let rate_str = rate.as_human_readable("/s");
        let percentage = if self.total_size > 0 {
            bytes_transferred as f64 / self.total_size as f64 * 100.0
        } else {
            0.0
        };
        let elapsed_str = elapsed_secs._to_string();
        
        let eta_str = if rate > 0.0 && self.total_size > 0 {
            let remaining_bytes = self.total_size.saturating_sub(bytes_transferred as u64);
            let eta_secs = (remaining_bytes as f64 / rate) as u64;
            eta_secs._to_string()
        } else {
            "--:--:--".to_string()
        };

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("transferred".to_string(), transferred_str);
        vars.insert("rate".to_string(), rate_str);
        vars.insert("percentage".to_string(), format!("{:.1}%", percentage));
        vars.insert("elapsed".to_string(), elapsed_str);
        vars.insert("eta".to_string(), eta_str);
        vars.insert("total".to_string(), (self.total_size as f64).as_human_readable(""));
        
        self.output.set_message(
            strfmt(
                &format!(
                    "{}{}{}{}{}{}{}{}{}{}{}{}",
                    "{transferred}".to_string().white().bold(),
                    "/".to_string().white(),
                    "{total} ".to_string().white().bold(),
                    "(".to_string().white(),
                    "{percentage}".to_string().green().bold(),
                    ") ".to_string().white(),
                    "{rate}".to_string().cyan().bold(),
                    " [".to_string().white(),
                    "{elapsed}".to_string().yellow().bold(),
                    "<".to_string().white(),
                    "{eta}".to_string().yellow().bold(),
                    "]".to_string().white(),
                ),
                &vars,
            )
            .unwrap(),
        );
    }
}

// 🏁 🚀📡🎯▶️🔽🔼
