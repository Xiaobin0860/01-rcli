use clap::Parser;
use std::str::FromStr;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Parser)]
pub struct JwtOpts {
    #[command(subcommand)]
    pub cmd: JwtSubCommand,
}

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[command(about = "Encode a JWT token")]
    Encode(JwtEncodeOpts),

    #[command(about = "Verify a JWT token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtEncodeOpts {
    #[arg(long)]
    pub aud: String,

    #[arg(long)]
    pub sub: String,

    #[arg(long, default_value = "1d", value_parser = parse_offset_date_time)]
    pub exp: OffsetDateTime,

    #[arg(long, value_parser = parse_jwt_alg, default_value = "HS256")]
    pub alg: JwtAlg,

    #[arg(long, default_value_t = false)]
    pub encode_secret: bool,
}

fn parse_offset_date_time(s: &str) -> Result<OffsetDateTime, &'static str> {
    let unit = &s[s.len() - 1..];
    let i: i64 = s[..s.len() - 1].parse().map_err(|_| "invalid duration")?;
    let dur = match unit {
        "m" => Duration::minutes(i),
        "h" => Duration::hours(i),
        "d" => Duration::days(i),
        _ => return Err("invalid duration unit"),
    };
    Ok(OffsetDateTime::now_utc() + dur)
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,

    #[arg(long)]
    pub aud: String,

    #[arg(long)]
    pub sub: String,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum JwtAlg {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
}

impl FromStr for JwtAlg {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HS256" => Ok(JwtAlg::HS256),
            "HS384" => Ok(JwtAlg::HS384),
            "HS512" => Ok(JwtAlg::HS512),
            "RS256" => Ok(JwtAlg::RS256),
            "RS384" => Ok(JwtAlg::RS384),
            "RS512" => Ok(JwtAlg::RS512),
            "ES256" => Ok(JwtAlg::ES256),
            "ES384" => Ok(JwtAlg::ES384),
            "ES512" => Ok(JwtAlg::ES512),
            _ => Err("invalid JWT algorithm"),
        }
    }
}

fn parse_jwt_alg(s: &str) -> Result<JwtAlg, &'static str> {
    s.parse()
}
