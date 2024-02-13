use crate::commands::PunchDirection;
use crate::Config;
use chrono::{Datelike, Timelike};
use clap::Parser;
use csv::{Reader, ReaderBuilder, Terminator, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    date: String,
    weekday: String,
    in_time: String,
    out_time: String,
    workinghours: f64,
    hours: f64,
}

pub struct Timesheet {
    timesheet_path: PathBuf,
    config: Config,
}

impl Timesheet {
    pub fn new(timesheet_path: PathBuf, config: Config) -> Self {
        Self {
            timesheet_path,
            config,
        }
    }
    fn get_records(&self) -> Result<Vec<Record>, anyhow::Error> {
        let mut rdr = self.get_rdr()?;
        let records: Vec<Record> = rdr.deserialize().map(|r| r.unwrap()).collect();
        if records.len() > 0 {
            return Ok(records);
        }

        let date = chrono::Local::now().date_naive();
        Ok(vec![Record {
            date: date.format(&*self.config.date_format).to_string(),
            weekday: date.weekday().to_string(),
            in_time: "".to_string(),
            out_time: "".to_string(),
            workinghours: 0.0,
            hours: 0.0,
        }])
    }

    fn get_rdr(&self) -> csv::Result<Reader<File>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path(&self.timesheet_path);
        return rdr;
    }

    fn get_wtr(&self) -> csv::Result<Writer<File>> {
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .terminator(Terminator::CRLF)
            .from_path(&self.timesheet_path);
        return wtr;
    }

    pub fn write_today_in(&self, in_time: &str) -> Result<(), csv::Error> {
        if let Ok(mut records) = self.get_records() {
            let mut wtr = self.get_wtr()?;
            let date = chrono::Local::now().date_naive();
            for mut record in &mut records {
                if chrono::NaiveDate::from_str(&*record.date) == Ok(date) {
                    record.in_time = in_time.to_string();
                }
                wtr.serialize(record)?;
            }
        }
        Ok(())
    }

    pub fn write_today_out(&self, out_time: &str) -> Result<(), csv::Error> {
        if let Ok(mut records) = self.get_records() {
            let mut wtr = self.get_wtr()?;
            let date = chrono::Local::now().date_naive();
            for mut record in &mut records {
                if chrono::NaiveDate::from_str(&*record.date) == Ok(date) {
                    record.out_time = out_time.to_string();
                    record.hours = self.calc_worked_time(&record.in_time, &record.out_time);
                }
                wtr.serialize(record)?;
            }
        }
        Ok(())
    }

    fn calc_worked_time(&self, in_time: &str, out_time: &str) -> f64 {
        let in_naivetime = chrono::NaiveTime::from_str(in_time).unwrap();
        let out_naivetime = chrono::NaiveTime::from_str(out_time).unwrap();

        (out_naivetime - in_naivetime).num_minutes() as f64 / 60.0
    }
}
