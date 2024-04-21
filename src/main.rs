use rcli::{
    convert_csv, gen_pass, process_decode, process_encode, B64SubCommand, Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse_args();

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
        SubCommand::B64(opts) => match opts.cmd {
            B64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            B64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
    }

    Ok(())
}
