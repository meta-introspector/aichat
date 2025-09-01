use anyhow::Result;

use crate::config::GlobalConfig;
use crate::repl::Repl;

pub async fn start_interactive(config: &GlobalConfig) -> Result<()> {
    let mut repl: Repl = Repl::init(config)?;
    repl.run().await
}
