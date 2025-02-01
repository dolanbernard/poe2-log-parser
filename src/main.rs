use std::{fs::File, io::Write};

use clap::Parser;

use args::Command;

mod args;
mod log_reader;

fn main() {
    let config = args::Args::parse();
    match config.command {
        Command::Clean { output_filename } => {
            let mut writer: Box<dyn Write> = Box::new(File::create(output_filename).unwrap());
            log_reader::clean(&config.log_file, &mut writer);
        }
        Command::Find {
            output_filename,
            character_name,
            regex_string 
        } => {
            let mut writer: Box<dyn Write> = match output_filename {
                Some(filename) => Box::new(File::create(&filename).unwrap()),
                None => Box::new(std::io::stdout()),
            };
            log_reader::find(&config.log_file, &character_name, &regex_string, &mut writer);
        },
    }
}
