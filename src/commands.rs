use crate::Config;
use chrono::{prelude::*, Duration, TimeDelta};
use clap::{Args, Subcommand};
use std::str::FromStr;
use toml::value::Time;

use crate::execute::Execute;
use crate::timesheet::{Record, Timesheet};

#[derive(Subcommand)]
pub enum PunchDirection {
    In(PunchArgs),
    Out(PunchArgs),
    Stats(StatsArgs),
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
            PunchDirection::Stats(args) => {
                println!("preparing working statistics...");
                let date = match &args.month {
                    Some(m) => chrono::Local::now()
                        .with_month(m.parse().unwrap_or(1))
                        .unwrap(),
                    None => chrono::Local::now(),
                };
                println!("for month: {}", date.format("%B").to_string());

                let timesheet_manager =
                    Timesheet::new(config.app_path.join("timesheet.csv"), config);
                let records: Vec<Record> = timesheet_manager.get_records().unwrap();
                let mut total_hours: TimeDelta = chrono::TimeDelta::minutes(0);
                let mut required_hours: TimeDelta = chrono::TimeDelta::minutes(0);
                for record in records {
                    if record.naive_date().month() == date.month() {
                        total_hours += chrono::TimeDelta::minutes((record.hours * 60.) as i64);
                        required_hours +=
                            chrono::TimeDelta::minutes((record.workinghours * 60.) as i64)
                    }
                }
                let diff = required_hours.num_minutes() as f64 / 60.
                    - total_hours.num_minutes() as f64 / 60.;
                println!(
                    "{}/{} hours; diff -> {}h",
                    total_hours.num_minutes() as f64 / 60.,
                    required_hours.num_minutes() as f64 / 60.,
                    diff
                );

                Ok(())
            }
        }
    }
}
