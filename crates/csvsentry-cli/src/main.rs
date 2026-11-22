use anyhow::Result;
use clap::{Parser, Subcommand};
use csvsentry_core::{
    domain::settings::Settings, services::validation_service::validate_from_settings_to_json_output,
};
use std::path::PathBuf;

use crate::helpers::file_paths::resolve_json_output_path;
mod helpers;

#[derive(Parser)]
#[command(name = "csvsentry", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        csv_filepath: PathBuf,

        #[arg(short, long)]
        settings_output_path: Option<PathBuf>,

        #[arg(short, long, default_value_t = ',')]
        delimiter: char,
    },
    Validate {
        settings_filepath: PathBuf,
        output_filepath: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate {
            csv_filepath,
            settings_output_path,
            delimiter,
        } => {
            println!("Generating baseconfiguration for {:?}", csv_filepath);
            let output_path =
                resolve_json_output_path(settings_output_path, "settings".to_string())?;

            let delimiter = u8::try_from(delimiter).map_err(|_| {
                anyhow::anyhow!("delimter '{}' is not a valid ASCII byte", delimiter)
            })?;

            Settings::create_settings_from_csv_file(csv_filepath, output_path, delimiter)?
        }
        Commands::Validate {
            settings_filepath,
            output_filepath,
        } => {
            println!(
                "Starting validation based on settings from {:?}",
                settings_filepath
            );

            let output_filepath = resolve_json_output_path(output_filepath, "results".to_string())?;
            let result_path =
                validate_from_settings_to_json_output(settings_filepath, output_filepath)?;
            println!(
                "Validation finished, result was written to {:?}",
                result_path
            );
        }
    }
    Ok(())
}
