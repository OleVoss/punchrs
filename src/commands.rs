use crate::Config;
use chrono::{prelude::*, Duration, TimeDelta};
use clap::{Args, Subcommand};
use std::str::FromStr;
use toml::value::Time;

use crate::execute::Execute;
use crate::timesheet::{Record, Timesheet};

#[derive(Subcommand)]
pub enum PunchDirection {
    #[command(about = "Punch in for today")]
    In(PunchArgs),
    #[command(about = "Punch out for today")]
    Out(PunchArgs),
    Stats(StatsArgs),
    #[command(about = "Displays a table with all entries in your .csv file")]
    Print,
}

#[derive(Args, Clone)]
pub struct PunchArgs {
    #[arg(required = true, help = "Time to save. (e.g. 8:00)")]
    time: String,
    #[arg(short, long, help = "Set a custom date. (e.g. 04.12.24)")]
    date: Option<String>,
    #[arg(short, long, help = "Set a custom target time for the entry.")]
    workinghours: Option<f64>,
}

#[derive(Args, Clone)]
pub struct StatsArgs {
    #[arg(short, long)]
    month: Option<String>,
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
