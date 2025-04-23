// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Hugo Melder

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use plist;
use std::fs;

mod xcstrings;

#[derive(Parser)]
#[command(name = "xcstringstool")]
#[command(about = "Work with .xcstrings files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints all string keys represented in an xcstrings file
    Print,

    /// Produces build products for an .xcstrings file
    Compile {
        /// The path to the .xcstrings file to compile
        input_file: String,

        /// The directory to place output files
        #[arg(short, long)]
        output_directory: String,

        /// Output format
        #[arg(short = 'f', long, default_value = "stringsAndStringsdict")]
        format: String,

        /// Language(s) to compile
        #[arg(short = 'l', long)]
        language: Vec<String>,

        /// Serialization format
        #[arg(long, default_value = "text")]
        serialization_format: String,

        /// Perform a dry run
        #[arg(long)]
        dry_run: bool,
    },

    /// Updates an .xcstrings file based on .stringsdata files
    Sync,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Print => {
            println!("Printing strings...");
        }
        Commands::Compile {
            input_file,
            output_directory,
            format,
            language,
            serialization_format,
            dry_run,
        } => {
            println!(
                "Compiling {} into {} with format {}, serialization_format {}, and languages {:?} ...",
                input_file, output_directory, format, serialization_format, language
            );
            if dry_run {
                println!("Dry run: true");
            }

            let xcstrings_content =
                fs::read_to_string(input_file).context("Failed to read xcstrings file")?;
            let parsed = xcstrings::deserialize(xcstrings_content.as_str())
                .context("Failed to parse xcstrings file")?;

            let strings = parsed.strings_for_localization("en");

            let file = fs::File::create("Localizable.strings")?;
            plist::to_writer_xml(file, &strings).context("Write strings file")?;

            println!(
                "Parsed: {:?} with source languge {}",
                parsed, parsed.source_language
            );
        }
        Commands::Sync => {
            println!("Syncing strings...");
        }
    }

    Ok(())
}
