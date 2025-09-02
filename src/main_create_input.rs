use anyhow::{bail, Result};

use crate::config::{GlobalConfig, Input};
use crate::utils::AbortSignal;

pub struct CreateInputArgs<'a> {
    pub config: &'a GlobalConfig,
    pub text: Option<String>,
    pub file: &'a [String],
    pub abort_signal: AbortSignal,
}

pub async fn create_input(args: CreateInputArgs<'_>) -> Result<Input> {
    let input = if args.file.is_empty() {
        Input::from_str(args.config, &args.text.unwrap_or_default(), None)
    } else {
        Input::from_files_with_spinner(
            args.config,
            &args.text.unwrap_or_default(),
            args.file.to_vec(),
            None,
            args.abort_signal,
        )
        .await?
    };
    if input.is_empty() {
        bail!("No input");
    }
    Ok(input)
}