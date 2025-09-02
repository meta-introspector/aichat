use anyhow::Result;

use crate::client::{call_chat_completions, call_chat_completions_streaming};
use crate::config::{GlobalConfig, Input};
use crate::utils::{AbortSignal, IS_STDOUT_TERMINAL};

pub struct StartDirectiveArgs {
    pub config: GlobalConfig,
    pub input: Input,
    pub code_mode: bool,
    pub abort_signal: AbortSignal,
}

#[async_recursion::async_recursion]
pub async fn start_directive(args: StartDirectiveArgs) -> Result<()> {
    let client = args.input.create_client().await?;
    let extract_code = !*IS_STDOUT_TERMINAL && args.code_mode;
    args.config.write().before_chat_completion(&args.input)?;
    let (output, tool_results) = if !args.input.stream() || extract_code {
        call_chat_completions(
            &args.input,
            true,
            extract_code,
            client.as_ref(),
            args.abort_signal.clone(),
        )
        .await?
    } else {
        call_chat_completions_streaming(
            &args.input,
            client.as_ref(),
            args.abort_signal.clone(),
        )
        .await?
    };
    args.config
        .write()
        .after_chat_completion(&args.input, &output, &tool_results)?;

    if !tool_results.is_empty() {
        start_directive(
            StartDirectiveArgs {
                config: args.config.clone(),
                input: args.input.merge_tool_results(output, tool_results),
                code_mode: args.code_mode,
                abort_signal: args.abort_signal.clone(),
            }
        )
        .await?;
    }

    args.config.write().exit_session()?;
    Ok(())
}