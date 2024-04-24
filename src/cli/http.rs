use std::path::PathBuf;

use super::verify_path;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct HttpOpts {
    #[command(subcommand)]
    pub cmd: HttpSubCommand,
}

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".", value_parser = verify_path)]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
