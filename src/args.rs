use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short='i', long="log-file", required=true)]
    pub log_file: String,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Clean {
        #[arg(short='o', long="output", required=true)]
        output_filename: String,
    },
    Find {
        #[arg(short='o', long="output", required=false)]
        output_filename: Option<String>,
        #[arg(short='c', long="character", required=false)]
        character_name: Option<String>,
        #[arg(required=true)]
        regex_string: String,
    },
}
