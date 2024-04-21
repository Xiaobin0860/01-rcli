use rcli::{
    b64_decode, b64_encode, convert_csv, gen_pass, key_gen, text_sign, text_verify, B64SubCommand,
    Opts, SubCommand, TextSubCommand,
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
            B64SubCommand::Encode(opts) => b64_encode(&opts.input, opts.format)?,
            B64SubCommand::Decode(opts) => b64_decode(&opts.input, opts.format)?,
        },
        SubCommand::Text(opts) => match opts.cmd {
            TextSubCommand::Sign(opts) => text_sign(&opts.input, &opts.key)?,
            TextSubCommand::Verify(opts) => text_verify(&opts.input, &opts.key, &opts.sig)?,
            TextSubCommand::Generate(opts) => key_gen(opts.format, &opts.output_path)?,
        },
    }

    Ok(())
}
