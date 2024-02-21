mod cli;
mod timesheet;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::execute::Execute;
use cli::Cli;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, ExecutableCommand};
use directories::BaseDirs;
use ratatui::backend::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use serde::Deserialize;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::{env, io};
use timesheet::check_timesheet;

// TODO: Anyhow error handling, but in good

const CONFIG_FILE_NAME: &str = "punchrs/config.toml";

#[derive(Clone, Default, Deserialize, Debug)]
struct Config {
    time_format: String,
    date_format: String,
    app_path: PathBuf,
    break_min: i32,
    work_hours: WorkdayHours,
    work_hours_month: i32,
}

#[derive(Deserialize, Default, Debug, Clone, Copy)]
struct WorkdayHours {
    monday: Option<f64>,
    tuesday: Option<f64>,
    wednesday: Option<f64>,
    thursday: Option<f64>,
    friday: Option<f64>,
    saturday: Option<f64>,
    sunday: Option<f64>,
}

impl WorkdayHours {
    pub fn get(&self, weekday: &str) -> Option<f64> {
        match weekday {
            "Mon" => self.monday,
            "Tue" => self.tuesday,
            "Wed" => self.wednesday,
            "Thu" => self.thursday,
            "Fri" => self.friday,
            "Sat" => self.saturday,
            "Sun" => self.sunday,
            _ => None,
        }
    }
}

fn get_config(path: PathBuf) -> Result<Config, anyhow::Error> {
    let mut config_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    let config: Config = toml::from_str(&config_content)?;
    return Ok(config);
}

fn main() -> Result<()> {
    if env::args().count() > 1 {
        let cli = Cli::parse();

        let dir = BaseDirs::new().unwrap();
        let config_path = dir.config_local_dir().join("punchrs");

        // config check
        // TODO: extract check and retrieval into single method
        if !config_path.exists() {
            print!(
                "{} does not exists. \nCreate new config? (y/N): ",
                config_path.display()
            );
            io::stdout().flush()?;

            let mut user_choice = String::new();
            match io::stdin().read_line(&mut user_choice) {
                Ok(_) => {
                    if user_choice.chars().next() == Some('y') {
                        create_dir_all(config_path).expect("Error creating the directory.");
                    } else {
                        return Ok(());
                    }
                }
                Err(_) => return Ok(()),
            }
        }
        let config = get_config(dir.config_local_dir().join(Path::new(CONFIG_FILE_NAME)))?;
        let timesheet_path = config.app_path.join("timesheet.csv");
        check_timesheet(timesheet_path)?;

        cli.command.execute(config)?;
    } else {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        loop {
            // draw
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            })?;

            // handle events
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        // cleanup
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
    }
    Ok(())
}
