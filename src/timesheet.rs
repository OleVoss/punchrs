use crate::Config;
use chrono::{Datelike, NaiveDate};
use csv::{Reader, ReaderBuilder, Terminator, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Record {
    pub date: String,
    pub weekday: String,
    pub in_time: String,
    pub out_time: String,
    pub workinghours: f64,
    pub hours: f64,
}

impl Record {
    pub fn naive_date(&self) -> NaiveDate {
        return chrono::NaiveDate::from_str(&self.date).unwrap();
    }
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
    pub fn get_records(&self) -> Result<Vec<Record>, anyhow::Error> {
        let mut rdr = self.get_rdr()?;
        let mut records: Vec<Record> = rdr.deserialize().map(|r| r.unwrap()).collect();
        if records
            .iter()
            .any(|r| r.date == chrono::Local::now().date_naive().format("%F").to_string())
        {
            return Ok(records);
        }
        let date = chrono::Local::now().date_naive();
        records.push(Record {
            date: date.format(&*self.config.date_format).to_string(),
            weekday: date.weekday().to_string(),
            in_time: "".to_string(),
            out_time: "".to_string(),
            workinghours: 0.0,
            hours: 0.0,
        });
        Ok(records)
    }

    pub fn get_today(&self) -> Result<Record, anyhow::Error> {
        let mut rdr = self.get_rdr()?;
        let records: Vec<Record> = rdr.deserialize().map(|r| r.unwrap()).collect();
        records
            .into_iter()
            .filter(|r| r.date == chrono::Local::now().date_naive().format("%F").to_string())
            .next()
            .map(|r| Ok(r))
            .unwrap()
    }

    fn get_rdr(&self) -> csv::Result<Reader<File>> {
        let rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path(&self.timesheet_path);
        return rdr;
    }

    fn get_wtr(&self) -> csv::Result<Writer<File>> {
        let wtr = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .terminator(Terminator::CRLF)
            .from_path(&self.timesheet_path);
        return wtr;
    }

    pub fn write_today_in(&self, in_time: &str, workinghours: f64) -> Result<(), csv::Error> {
        if let Ok(records) = self.get_records() {
            let mut wtr = self.get_wtr()?;
            let date = chrono::Local::now().date_naive();
            for mut record in records {
                if chrono::NaiveDate::from_str(&*record.date) == Ok(date) {
                    record.in_time = in_time.to_string();
                    record.workinghours = workinghours;
                }
                wtr.serialize(record)?;
            }
        }
        Ok(())
    }

    pub fn write_today_out(&self, out_time: &str, break_minutes: i32) -> Result<(), csv::Error> {
        if let Ok(mut records) = self.get_records() {
            let mut wtr = self.get_wtr()?;
            let date = chrono::Local::now().date_naive();
            for record in &mut records {
                if chrono::NaiveDate::from_str(&*record.date) == Ok(date) {
                    record.out_time = out_time.to_string();
                    record.hours = self.calc_worked_time(&record.in_time, &record.out_time)
                        - break_minutes as f64 / 60.;
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

pub fn check_timesheet(timesheet_path: PathBuf) -> anyhow::Result<()> {
    if !timesheet_path.exists() {
        print!(
            "{} does not exist. Create file? (y/N): ",
            timesheet_path.display()
        );
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
            Err(e) => Err(anyhow::Error::from(e)),
        }
    } else {
        Ok(())
    }
}
