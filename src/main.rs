use rcli::{convert_csv, gen_pass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse_args();
    println!("{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            convert_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::Pass(opts) => gen_pass(
            opts.length,
            opts.no_lower,
            opts.no_upper,
            opts.no_number,
            opts.no_symbol,
        )?,
    }

    Ok(())
}
