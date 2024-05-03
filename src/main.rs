use rcli::{
    b64_decode, b64_encode, base64_encode, convert_csv, data_decrypt, data_encrypt, gen_pass,
    get_reader, jwt_encode, jwt_verify, key_gen, process_http_serve, text_sign, text_verify,
    B64SubCommand, HttpSubCommand, JwtClaims, Opts, SubCommand, TextSubCommand,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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
            TextSubCommand::Encrypt(opts) => {
                let mut data_reader = get_reader(&opts.input)?;
                let encrypted = data_encrypt(&mut data_reader, &opts.key)?;
                let b64 = base64_encode(encrypted);
                println!("{b64}");
            }
            TextSubCommand::Decrypt(opts) => {
                let mut data_reader = get_reader(&opts.input)?;
                let decrypted = data_decrypt(&mut data_reader, &opts.key)?;
                let decrypted = String::from_utf8(decrypted)?;
                println!("{decrypted}");
            }
        },
        SubCommand::Http(opts) => match opts.cmd {
            HttpSubCommand::Serve(opts) => process_http_serve(&opts.dir, opts.port).await?,
        },
        SubCommand::Jwt(opts) => match opts.cmd {
            rcli::JwtSubCommand::Encode(opts) => {
                let claims = JwtClaims::new(
                    opts.aud.clone(),
                    opts.sub.clone(),
                    opts.exp.unix_timestamp(),
                );
                let jwt = jwt_encode(&claims, opts.alg)?;
                println!("{jwt}");
            }
            rcli::JwtSubCommand::Verify(opts) => {
                println!("{}", jwt_verify(&opts.token, &opts.aud, &opts.sub)?);
            }
        },
    }

    Ok(())
}
