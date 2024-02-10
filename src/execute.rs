use anyhow::Result;
use crate::Config;

pub trait Execute {
    fn execute(&self, config: Config) -> Result<()>;
}
