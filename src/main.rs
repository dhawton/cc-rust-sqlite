use anyhow::{Result, bail};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 110];
            file.read_exact(&mut header)?;

            let page_size = u16::from_be_bytes([header[16], header[17]]);
            let table_count: u16 = u16::from_be_bytes([header[103], header[104]]); 

            println!("database page size: {}", page_size);
            println!("number of tables: {}", table_count);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
