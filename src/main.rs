use rcli::{
    b64_decode, b64_encode, convert_csv, gen_pass, get_reader, key_gen, text_sign, text_verify,
    B64SubCommand, Opts, SubCommand, TextSubCommand,
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
        SubCommand::Pass(opts) => {
            let pass = gen_pass(
                opts.length,
                opts.no_lower,
                opts.no_upper,
                opts.no_number,
                opts.no_symbol,
            )?;
            println!("{pass}");
        }
        SubCommand::B64(opts) => match opts.cmd {
            B64SubCommand::Encode(opts) => b64_encode(&opts.input, opts.format)?,
            B64SubCommand::Decode(opts) => b64_decode(&opts.input, opts.format)?,
        },
        SubCommand::Text(opts) => match opts.cmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let encoded = text_sign(opts.format, &mut reader, &opts.key)?;
                println!("{encoded}");
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let valid = text_verify(opts.format, &mut reader, &opts.key, &opts.sig)?;
                println!("{valid}");
            }
            TextSubCommand::Generate(opts) => key_gen(opts.format, &opts.output_path)?,
        },
    }

    Ok(())
}
