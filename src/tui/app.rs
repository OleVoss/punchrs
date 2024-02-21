use std::io::{stdout, Stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, List, Paragraph},
    CompletedFrame, Frame, Terminal,
};

use crate::Config;

pub struct PunchrsApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    config: Config,
}

impl PunchrsApp {
    pub fn new(config: Config) -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            config,
        }
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.terminal.clear()?;

        loop {
            // draw
            let completed_frame = self.ui()?;

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
        Ok(())
    }

    fn ui(&mut self) -> Result<CompletedFrame, anyhow::Error> {
        let frame = self.terminal.draw(|frame| {
            // app layout
            let layout_app = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(3),
                    Constraint::Fill(1),
                    Constraint::Length(3),
                ])
                .split(frame.size());

            // main layout
            let layout_main = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Min(24), Constraint::Fill(10)])
                .split(layout_app[1]);

            let layout_month = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(5), Constraint::Length(19)])
                .split(layout_main[0]);

            // ## widgets #####################################
            // header
            frame.render_widget(
                Paragraph::new("Header")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL)),
                layout_app[0],
            );

            // calender
            frame.render_widget(
                List::new([
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                    "Dec",
                ])
                .block(Block::default().borders(Borders::ALL).title("Month")),
                layout_month[0],
            );
            frame.render_widget(
                Paragraph::new("02 Mon 12.01.2024")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Date")),
                layout_month[1],
            );

            // stats
            frame.render_widget(
                Paragraph::new("Stats").block(Block::default().borders(Borders::ALL)),
                layout_main[1],
            );

            // footer
            frame.render_widget(
                Paragraph::new("<commands>")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL)),
                layout_app[2],
            );
        })?;
        Ok(frame)
    }
}
