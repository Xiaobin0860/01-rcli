use std::fs;

use csv::Reader as CsvReader;
use serde::{Deserialize, Serialize};

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut rdr = CsvReader::from_path(input)?;
    let mut players = Vec::new();
    for player in rdr.deserialize() {
        let player: Player = player?;
        println!("{player:?}");
        players.push(player);
    }
    let json = serde_json::to_string_pretty(&players)?;
    fs::write(output, json)?;
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
