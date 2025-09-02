use anyhow::Result;

use crate::config::GlobalConfig;
use crate::repl::Repl;

pub struct StartInteractiveArgs {
    pub config: GlobalConfig,
}

pub async fn start_interactive(args: StartInteractiveArgs) -> Result<()> {
    let mut repl: Repl = Repl::init(&args.config)?;
    repl.run().await
}