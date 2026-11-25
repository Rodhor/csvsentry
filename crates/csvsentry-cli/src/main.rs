use anyhow::{Context, Result};
use axoupdater::AxoUpdater;
use clap::{Parser, Subcommand};
use csvsentry_core::{
    domain::settings::Settings, services::validation_service::validate_from_settings,
};
use std::path::PathBuf;
use std::sync::Arc;
use crate::helpers::{
    file_paths::resolve_json_output_path,
    output::{OutputFormat, write_results},
};
mod helpers;

#[derive(Parser)]
#[command(name = "csvsentry", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update,
    Generate {
        #[arg(short, long)]
        csv_filepath: PathBuf,

        #[arg(short, long)]
        settings_output_path: Option<PathBuf>,

        #[arg(short, long, default_value_t = ',')]
        delimiter: char,

        #[arg(short, long, default_value = "false")]
        explore_types: bool,

        #[arg(short, long)]
        sample_size: Option<usize>
    },
    Validate {
        #[arg(short, long)]
        settings_filepath: PathBuf,

        #[arg(short, long)]
        output_filepath: Option<PathBuf>,

        #[arg(short, long, value_enum, default_value = "json")]
        format: OutputFormat,

        #[arg(short, long, default_value = "false")]
        verbose_result_output: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Update => {
            let mut updater = AxoUpdater::new_for("csvsentry-cli");
            updater.load_receipt()
                .context("no install receipt found; `csvsentry update` works for shell/Powershell installs only")?;

            if !updater.check_receipt_is_for_this_executable()? {
                anyhow::bail!("this binary was not installed by the csvsentry installer")
            }

            match updater.run_sync()? {
                Some(result) => println!("Updated csvsentry to {}", result.new_version),
                None => println!("csvsentry is already up to date"),
            }
        }
        Commands::Generate {
            csv_filepath,
            settings_output_path,
            delimiter,
            explore_types,
            sample_size,

        } => {
            println!("Generating baseconfiguration for {:?}", csv_filepath);
            let output_path = resolve_json_output_path(
                settings_output_path,
                OutputFormat::Json,
                "settings".to_string(),
            )?;

            let delimiter = u8::try_from(delimiter).map_err(|_| {
                anyhow::anyhow!("delimter '{}' is not a valid ASCII byte", delimiter)
            })?;

            Settings::create_settings_from_csv_file_with_options(csv_filepath, output_path, delimiter, explore_types, sample_size)?
        }
        Commands::Validate {
            settings_filepath,
            output_filepath,
            format,
            verbose_result_output,
        } => {
            println!(
                "Starting validation based on settings from {:?}",
                settings_filepath
            );

            let path = resolve_json_output_path(output_filepath, format, "results".to_string())?;
            let results = validate_from_settings(settings_filepath)?;
            write_results(results, &path, format, verbose_result_output)?;
            println!("Validation finished, result was written to {:?}", path);
        }
    }
    Ok(())
}
