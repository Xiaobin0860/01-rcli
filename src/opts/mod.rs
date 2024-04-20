use clap::Parser;

mod csv_opts;
pub use csv_opts::*;

mod pass_opts;
pub use pass_opts::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(about = "Show CSV, or convert CSV to other formats.")]
    Csv(CsvOpts),

    #[command(about = "Generate password.")]
    Pass(PassOpts),
}
