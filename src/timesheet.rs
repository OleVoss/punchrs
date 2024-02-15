use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use chrono::{Datelike, Timelike};
use clap::Parser;
use csv::{Reader, ReaderBuilder, Terminator, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};
use crate::commands::PunchDirection;


#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Record {
    date: String,
    weekday: String,
    in_time: String,
    out_time: String,
    hours: i64,
}

pub struct Timesheet {
    timesheet_path: PathBuf,
    date_format: String,
    time_format: String,
}

impl Timesheet {
    pub fn new(timesheet_path: PathBuf, date_format: String, time_format: String) -> Self {
        Self {
            timesheet_path,
            date_format,
            time_format
        }
    }

    pub fn print_records(&self) {
        let records = self.get_records().unwrap();
        let table = Table::new(records).to_string();
        println!("{}", table);
    }

    fn get_records(&self) -> Result<Vec<Record>, anyhow::Error> {
        let mut rdr = self.get_rdr()?;
        let records: Vec<Record> = rdr.deserialize().map(|r| r.unwrap()).collect();
        if records.len() > 0 {
            return Ok(records)
        }

        let date = chrono::Local::now().date_naive();
        Ok(vec![Record {
            date: date.format(&*self.date_format).to_string(),
            weekday: date.weekday().to_string(),
            in_time: "".to_string(),
            out_time: "".to_string(),
            hours: 0,
        }])
    }

    fn get_rdr(&self) -> csv::Result<Reader<File>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path(&self.timesheet_path);
        return rdr
    }

    fn get_wtr(&self) -> csv::Result<Writer<File>> {
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .terminator(Terminator::CRLF)
            .from_path(&self.timesheet_path);
        return wtr
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

    fn calc_worked_time(&self, in_time: &str, out_time: &str) -> i64 {
        let in_naivetime = chrono::NaiveTime::from_str(in_time).unwrap();
        let out_naivetime = chrono::NaiveTime::from_str(out_time).unwrap();

        return (out_naivetime - in_naivetime).num_minutes()
    }
}

