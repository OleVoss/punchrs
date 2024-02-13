use crate::{get_config, Config};
use chrono::{prelude::*, TimeDelta};
use clap::{Args, Subcommand};
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    #[arg(short, long)]
    workinghours: Option<f64>,
}

impl Execute for PunchDirection {
    fn execute(&self, config: Config) -> anyhow::Result<()> {
        match self {
            PunchDirection::In(args) => {
                let workinghours = match args.workinghours {
                    Some(w) => w,
                    None => config
                        .work_hours
                        .get(&chrono::Local::now().weekday().to_string())
                        .unwrap(),
                };

                let timesheet_manager =
                    Timesheet::new(config.app_path.join("timesheet.csv"), config.clone());
                timesheet_manager.write_today_in(args.time.as_str(), workinghours)?;
                let naive_time = chrono::NaiveTime::from_str(&args.time).unwrap();

                println!(
                    "You are working from {} to {}.",
                    args.time,
                    (naive_time
                        + TimeDelta::hours(workinghours as i64)
                        + TimeDelta::minutes(config.break_min as i64))
                    .format("%H:%M")
                );
                Ok(())
            }
            PunchDirection::Out(args) => {
                let break_minutes = config.break_min;
                let timesheet_manager =
                    Timesheet::new(config.app_path.join("timesheet.csv"), config);
                timesheet_manager.write_today_out(args.time.as_str(), break_minutes)?;
                Ok(())
            }
        }
    }
}
