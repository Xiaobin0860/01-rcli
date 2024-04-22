use anyhow::Result;
use std::{
    fs::{self, File},
    io::{self, Read},
};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

pub fn get_content(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn get_data(path: &str) -> Result<Vec<u8>> {
    Ok(fs::read(path)?)
}
