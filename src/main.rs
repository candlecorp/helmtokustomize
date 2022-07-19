use clap::Parser;
use std::{fs::File, any::Any};
use std::io::prelude::*;
use std::collections::HashMap;
use anyhow::{Context, Result};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    input_file: String,

    #[clap(short, long, value_parser)]
    output_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut in_file = File::open(args.input_file)?;
    let mut contents = String::new();
    in_file.read_to_string(&mut contents)?;

    let out_files = contents.split("---");

    for out_file in out_files {
        if out_file == "" {
            continue;
        }
        let mut content_type = HashMap::new();
        let content_map = serde_yaml::from_str(out_file)?;
        let kind = content_map.get("kind");
        let name = content_map.get("metadata");

        println!("{:?}",kind);
        println!("{:?}",name);

    }

    Ok(())
}