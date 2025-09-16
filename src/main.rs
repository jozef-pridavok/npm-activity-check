use anyhow::Result;
use clap::Parser;

mod config;
mod history;
mod npm;
mod output;
mod scoring;
mod types;

use config::Config;
use history::HistoryData;
use npm::NpmClient;
use output::{create_package_report, print_output};

macro_rules! verbose_println {
    ($config:expr, $($arg:tt)*) => {
        if $config.verbose {
            eprintln!("[VERBOSE] {}", format!($($arg)*));
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        
        let chain = e.chain().skip(1);
        for cause in chain {
            eprintln!("  Caused by: {cause}");
        }
        
        std::process::exit(1);
    }
    
    Ok(())
}

async fn run() -> Result<()> {
    let config = Config::parse();
    config.validate()?;

    let config = if let Some(config_path) = &config.config_file {
        verbose_println!(&config, "Loading configuration file: {}", config_path);
        let file_config = Config::from_toml(config_path)?;
        config.merge(file_config).with_defaults()
    } else {
        config.with_defaults()
    };

    let npm_client = NpmClient::new()?;

    verbose_println!(&config, "Fetching package data from NPM registry...");
    
    let package_info = npm_client.get_package_info(config.get_package()).await?;
    let weekly_downloads = npm_client.get_weekly_downloads(config.get_package()).await?;
    let monthly_downloads = npm_client.get_monthly_downloads(config.get_package()).await?;

    let (latest_version, last_publish_date) = npm_client
        .get_latest_version_info(&package_info)
        .ok_or_else(|| anyhow::anyhow!("Could not determine latest version"))?;

    let current_report = create_package_report(
        &config,
        &package_info,
        &latest_version,
        &last_publish_date,
        weekly_downloads,
        monthly_downloads,
    );

    if let Some(history_path) = &config.history {
        let existing_history = HistoryData::load(history_path, config.verbose)?;

        let new_history = HistoryData {
            last_data: current_report.clone(),
        };
        new_history.save(history_path, config.verbose)?;

        if let Some(check_field) = &config.check {
            verbose_println!(&config, "Checking field '{}' for changes", check_field);
            
            if let Some(history) = existing_history {
                let change_magnitude = history.calculate_change(&current_report, check_field)?;
                verbose_println!(&config, "Change magnitude for '{}': {}", check_field, change_magnitude);
                std::process::exit(change_magnitude as i32);
            } else {
                verbose_println!(&config, "No history exists, no change to compare (exit code: 0)");
                std::process::exit(0);
            }
        }
    } else if config.check.is_some() {
        anyhow::bail!("--check requires --history to be specified");
    }

    print_output(&config, &current_report)?;

    Ok(())
}
