mod commands;
mod execute;
mod timesheet;

use std::env::join_paths;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::Result;
use clap::Parser;
use directories::BaseDirs;
use commands::PunchDirection;
use execute::Execute;
use log::log;
use serde::{Deserialize, Deserializer};

// TODO: Anyhow error handling, but in good

const CONFIG_FILE_NAME: &str = "punchrs/config.toml";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: PunchDirection,
}

#[derive(Default, Deserialize, Debug)]
struct Config {
    time_format: String,
    date_format: String,
    app_path: PathBuf,
    break_min: i32,
    work_hours: WorkdayHours,
    work_hours_month: i32,
}

#[derive(Deserialize, Default, Debug)]
struct WorkdayHours {
    monday: Option<i32>,
    tuesday: Option<i32>,
    wednesday: Option<i32>,
    thursday: Option<i32>,
    friday: Option<i32>,
    saturday: Option<i32>,
    sunday: Option<i32>,
}

fn get_config(path: PathBuf) -> Result<Config, anyhow::Error> {
    let mut config_file = OpenOptions::new().write(true).read(true).create(true).open(path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    let config: Config = toml::from_str(&config_content)?;
    return Ok(config)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dir = BaseDirs::new().unwrap();
    let config_path = dir.config_local_dir().join("punchrs");

    // config check
    // TODO: extract check and retrieval into single method
    if !config_path.exists() {
        print!("{} does not exists. \nCreate new config? (y/N): ", config_path.display());
        io::stdout().flush()?;

        let mut user_choice = String::new();
        match io::stdin().read_line(&mut user_choice) {
            Ok(_) => {
                if user_choice.chars().next() == Some('y') {
                    create_dir_all(config_path).expect("Error creating the directory.");
                } else {
                    return Ok(())
                }
            }
            Err(_) => return Ok(())
        }
    }
    let config = get_config(dir.config_local_dir().join(Path::new(CONFIG_FILE_NAME)))?;
    let timesheet_path = config.app_path.join("timesheet.csv");
    check_timesheet(timesheet_path)?;

    cli.command.execute(config)?;
    Ok(())
}

fn check_timesheet(timesheet_path: PathBuf) -> anyhow::Result<()> {
    if !timesheet_path.exists() {
        print!("{} does not exist. Create file? (y/N): ", timesheet_path.display());
        io::stdout().flush()?;

        let mut user_choice = String::new();
        match io::stdin().read_line(&mut user_choice) {
            Ok(_) => {
                if user_choice.chars().next() == Some('y') {
                    File::create(timesheet_path)?;
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(anyhow::Error::from(e))
        }
    } else {
        Ok(())
    }
}

