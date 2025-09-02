use anyhow::{bail, Result};
use inquire::Text;


use crate::client::{call_chat_completions, call_chat_completions_streaming};
use crate::config::{GlobalConfig, Input, EXPLAIN_SHELL_ROLE};
use crate::utils::{AbortSignal, append_to_shell_history, color_text, dimmed_text, read_single_key, run_command, set_text, IS_STDOUT_TERMINAL, Shell};
use std::process;

pub struct ShellExecuteArgs {
    pub config: GlobalConfig,
    pub shell: Shell,
    pub input: Input,
    pub abort_signal: AbortSignal,
}

#[async_recursion::async_recursion]
pub async fn shell_execute(args: ShellExecuteArgs) -> Result<()> {
    let client = args.input.create_client().await?;
    args.config.write().before_chat_completion(&args.input)?;
    let (eval_str, _) =
        call_chat_completions(&args.input, false, true, client.as_ref(), args.abort_signal.clone()).await?;

    args.config
        .write()
        .after_chat_completion(&args.input, &eval_str, &[])?;
    if eval_str.is_empty() {
        bail!("No command generated");
    }
    if args.config.read().dry_run {
        println!("{}", &eval_str);
        return Ok(());
    }
    if *IS_STDOUT_TERMINAL {
        let options = ["execute", "revise", "describe", "copy", "quit"];
        let _command = color_text(eval_str.trim(), nu_ansi_term::Color::Rgb(255, 165, 0));
        let first_letter_color = nu_ansi_term::Color::Cyan;
        let prompt_text =
            options
                .iter()
                .map(|v| format!("{}{}", color_text(&v[0..1], first_letter_color), &v[1..]))
                .collect::<Vec<String>>()
                .join(&dimmed_text(" | "));
        loop {
            println!("{{command}}");
            let answer_char =
                read_single_key(&['e', 'r', 'd', 'c', 'q'], 'e', &format!("{prompt_text}: "))?;

            match answer_char {
                'e' => {
                    debug!("{} {:?}", args.shell.cmd, &[&args.shell.arg, &eval_str]);
                    let code = run_command(&args.shell.cmd, &[&args.shell.arg, &eval_str], None)?;
                    if code == 0 && args.config.read().save_shell_history {
                        let _ = append_to_shell_history(&args.shell.name, &eval_str, code);
                    }
                    process::exit(code);
                }
                'r' => {
                    let _revision = Text::new("Enter your revision:").prompt()?;
                    let text = format!("{}\n{{revision}}", args.input.text());
                    let mut input = args.input; // Make input mutable
                    input.set_text(text);
                    return shell_execute(ShellExecuteArgs { config: args.config.clone(), shell: args.shell.clone(), input, abort_signal: args.abort_signal.clone() }).await;
                }
                'd' => {
                    let role = args.config.read().retrieve_role(EXPLAIN_SHELL_ROLE)?;
                    let input = Input::from_str(&args.config, &eval_str, Some(role));
                    if input.stream() {
                        call_chat_completions_streaming(
                            &input,
                            client.as_ref(),
                            args.abort_signal.clone(),
                        ).await?;
                    }
                    else {
                        call_chat_completions(
                            &input,
                            true,
                            false,
                            client.as_ref(),
                            args.abort_signal.clone(),
                        ).await?;
                    }
                    println!();
                    continue;
                }
                'c' => {
                    set_text(&eval_str)?;
                    println!("{}", dimmed_text("✓ Copied the command."));
                }
                _ => {}
            }
            break;
        }
    } else {
        println!("{{eval_str}}");
    }
    Ok(())
}