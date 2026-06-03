mod models;
mod mapper;

use clap::Parser;
use csv::{Reader, Writer};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use mapper::map_google_to_unifi;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input Google Contacts CSV file
    #[arg(short, long)]
    input: PathBuf,

    /// Output Unifi Talk CSV file
    #[arg(short, long, default_value = "unifi_talk_contacts.csv")]
    output: PathBuf,
}

fn process_contacts(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(input_path)?;
    let mut writer = Writer::from_path(output_path)?;

    for result in reader.deserialize::<HashMap<String, String>>() {
        let record = result?;
        if let Some(unifi_contact) = map_google_to_unifi(&record) {
            writer.serialize(unifi_contact)?;
        }
    }

    writer.flush()?;
    println!("Successfully converted contacts and saved to {}", output_path.display());
    
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = process_contacts(&args.input, &args.output) {
        eprintln!("Error processing contacts: {}", e);
        std::process::exit(1);
    }
}
