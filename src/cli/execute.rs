use crate::Config;
use anyhow::Result;

pub trait Execute {
    fn execute(&self, config: Config) -> Result<()>;
}
