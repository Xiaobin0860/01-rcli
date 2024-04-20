use std::fs;

use csv::Reader as CsvReader;
use serde::{Deserialize, Serialize};

use crate::OutputFormat;

pub fn convert_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut rdr = CsvReader::from_path(input)?;
    let mut players = Vec::new();
    for player in rdr.deserialize() {
        let player: Player = player?;
        players.push(player);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&players)?,
        OutputFormat::Yaml => serde_yaml::to_string(&players)?,
    };
    println!("{content}");
    fs::write(output, content)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
