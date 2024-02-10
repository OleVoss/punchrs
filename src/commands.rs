use std::path::{Path, PathBuf};
use chrono::prelude::*;
use clap::{Args, Subcommand};
use crate::{Config, get_config};

use crate::execute::Execute;
use crate::timesheet::Timesheet;

#[derive(Subcommand)]
pub enum PunchDirection {
    In(PunchArgs),
    Out(PunchArgs),
}

#[derive(Args, Clone)]
pub struct PunchArgs {
    #[arg(required = true)]
    time: String,
    #[arg(short, long)]
    date: Option<String>,
}

impl Execute for PunchDirection {
    fn execute(&self, config: Config) -> anyhow::Result<()> {
        match self {
            PunchDirection::In(args) => {
                let timesheet_manager = Timesheet::new(config.app_path.join("timesheet.csv"), config.date_format, config.time_format);
                timesheet_manager.write_today_in(args.time.as_str())?;
                Ok(())
            }
            PunchDirection::Out(args) => {
                let timesheet_manager = Timesheet::new(config.app_path.join("timesheet.csv"), config.date_format, config.time_format);
                timesheet_manager.write_today_out(args.time.as_str())?;
                Ok(())
            }
        }
    }
}