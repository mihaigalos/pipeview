use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Input file
    #[arg(long, short)]
    pub infile: Option<String>,
    /// Output file
    #[arg(long, short)]
    pub outfile: Option<String>,
    /// Silent mode
    #[arg(long)]
    pub silent: bool,
}
