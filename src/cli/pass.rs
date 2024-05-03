use crate::{gen_pass, CmdExecutor};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct PassOpts {
    #[arg(short, long, default_value_t = 12)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_upper: bool,

    #[arg(long, default_value_t = false)]
    pub no_lower: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExecutor for PassOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let pass = gen_pass(
            self.length,
            self.no_lower,
            self.no_upper,
            self.no_number,
            self.no_symbol,
        )?;
        println!("{pass}");
        Ok(())
    }
}
