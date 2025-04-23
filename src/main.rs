// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Hugo Melder

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    Print {
        /// The path to the .xcstrings file to print
        input_file: String,
    },

    /// Produces build products for an .xcstrings file
    Compile {
        /// The path to the .xcstrings file to compile
        input_file: String,

        /// The directory to place output files
        #[arg(short, long, value_parser = clap::value_parser!(PathBuf))]
        output_directory: PathBuf,

        /// Output format
        #[arg(short = 'f', long, default_value = "stringsAndStringsdict")]
        format: String,

        /// Language to compile
        #[arg(short = 'l', long)]
        language: Option<String>,

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
        Commands::Print { input_file } => {
            let xcstrings_content =
                fs::read_to_string(input_file).context("Failed to read xcstrings file")?;

            let parsed = xcstrings::deserialize(xcstrings_content.as_str())
                .context("Failed to parse xcstrings file")?;

            for (key, _) in &parsed.strings {
                println!("{}", key);
            }
        }
        Commands::Compile {
            input_file,
            output_directory,
            format,
            language,
            serialization_format,
            dry_run,
        } => {
            if format != "stringsAndStringsdict" {
                eprintln!(
                    "{}: Formats other than 'stringsAndStringsdict' are currently unsupported",
                    "warning".yellow().bold()
                );
            }

            let xcstrings_content =
                fs::read_to_string(input_file).context("Failed to read xcstrings file")?;

            // Parse the .xcstrings file
            let parsed = xcstrings::deserialize(xcstrings_content.as_str())
                .context("Failed to parse xcstrings file")?;

            // Retrieve mappings for all locales
            let all_strings;

            if language.is_none() {
                all_strings = parsed.all_strings();
            } else {
                let mut all_strings_map: HashMap<String, HashMap<String, String>> = HashMap::new();

                let l = language.unwrap();
                let mapping = parsed.strings_for_localization(l.as_str());
                all_strings_map.insert(l, mapping);

                all_strings = all_strings_map;
            }

            // Do not write files when doing a dry run
            if !dry_run {
                for (locale, mapping) in all_strings {
                    let lproj_path = &output_directory.join(locale + ".lproj");

                    // Create lproj directory if absent
                    fs::create_dir_all(lproj_path)?;

                    // Open a Localizable.strings file handle
                    let strings_path = &lproj_path.join("Localizable.strings");
                    let strings_file = fs::File::create(strings_path)
                        .context("Failed to create Localizable.strings file")?;

                    // Serialize the plist
                    if serialization_format == "text" {
                        plist::to_writer_xml(strings_file, &mapping)
                            .context("Write strings file")?;
                    } else if serialization_format == "binary" {
                        plist::to_writer_binary(strings_file, &mapping)
                            .context("Write strings file")?;
                    } else {
                        // TODO(hugo): Convert into error
                        eprintln!(
                            "{}: Unknown serialization format {}",
                            "error".red().bold(),
                            serialization_format
                        );
                        break;
                    }
                }
            }
        }
        Commands::Sync => {
            println!("{}: Sync is not implemented", "error".red().bold());
        }
    }

    Ok(())
}
