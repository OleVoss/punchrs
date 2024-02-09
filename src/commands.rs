use chrono::prelude::*;
use clap::{Args, Subcommand};

use crate::execute::Execute;

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
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            PunchDirection::In(args) => {
                println!("in time: {}", args.time);
                Ok(())
            }
            PunchDirection::Out(args) => {
                println!("out time: {}", args.time);
                Ok(())
            }
        }
    }
}
