use rcli::{gen_pass, process_csv, Opts, SubCommand};

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
            process_csv(&opts.input, &output, opts.format)?;
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
