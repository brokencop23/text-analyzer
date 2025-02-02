use std::fs;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn read_file(file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = 
    let contents = fs::read_to_string(file_path)?;
    println!("Content: {}", contents);
    Ok(())
}

