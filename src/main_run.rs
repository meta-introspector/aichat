use anyhow::{bail, Result};

use crate::cli::Cli;
use crate::client::{list_models, ModelType};
use crate::config::{list_agents, Config, GlobalConfig, CODE_ROLE, SHELL_ROLE, TEMP_SESSION_NAME};
use crate::utils::{*, IS_STDOUT_TERMINAL, SHELL};
use crate::serve;
use crate::main_create_input::create_input;
use crate::main_shell_execute::shell_execute;
use crate::main_start_directive::start_directive;
use crate::main_start_interactive::start_interactive;

pub struct RunArgs {
    pub config: GlobalConfig,
    pub cli: Cli,
    pub text: Option<String>,
}

pub async fn run(args: RunArgs) -> Result<()> {
    let abort_signal = create_abort_signal();

    if args.cli.sync_models {
        let url = args.config.read().sync_models_url.clone();
        return Config::sync_models(url.as_deref().unwrap_or_default(), abort_signal.clone()).await;
    }

    if args.cli.list_models {
        for model in list_models(&args.config.read(), ModelType::Chat) {
            println!("{}", model.id());
        }
        return Ok(());
    }
    if args.cli.list_roles {
        let _roles = Config::list_roles(true).join("\n");
        println!("{{roles}}");
        return Ok(());
    }
    if args.cli.list_agents {
        let _agents = list_agents().join("\n");
        println!("{{agents}}");
        return Ok(());
    }
    if args.cli.list_rags {
        let _rags = Config::list_rags().join("\n");
        println!("{{rags}}");
        return Ok(());
    }
    if args.cli.list_macros {
        let _macros = Config::list_macros().join("\n");
        println!("{{macros}}");
        return Ok(());
    }

    if args.cli.dry_run {
        args.config.write().dry_run = true;
    }

    if let Some(agent) = &args.cli.agent {
        let session = args.cli.session.as_ref().map(|v| match v {
            Some(v) => v.as_str(),
            None => TEMP_SESSION_NAME,
        });
        if !args.cli.agent_variable.is_empty() {
            args.config.write().agent_variables = Some(
                args.cli.agent_variable
                    .chunks(2)
                    .map(|v| (v[0].to_string(), v[1].to_string()))
                    .collect(),
            );
        }

        let ret = Config::use_agent(&args.config, agent, session.unwrap_or_default(), abort_signal.clone()).await;
        args.config.write().agent_variables = None;
        ret?;
    } else {
        if let Some(prompt) = &args.cli.prompt {
            args.config.write().use_prompt(prompt)?;
        } else if let Some(name) = &args.cli.role {
            args.config.write().use_role(name)?;
        } else if args.cli.execute {
            args.config.write().use_role(SHELL_ROLE)?;
        } else if args.cli.code {
            args.config.write().use_role(CODE_ROLE)?;
        }
        if let Some(session) = &args.cli.session {
            args.config
                .write()
                .use_session(session.as_ref().map(|v| v.as_str()))?;
        }
        if let Some(rag) = &args.cli.rag {
            Config::use_rag(&args.config, Some(rag), abort_signal.clone()).await?;
        }
    }
    if args.cli.list_sessions {
        let _sessions = args.config.read().list_sessions().join("\n");
        println!("{{sessions}}");
        return Ok(());
    }
    if let Some(model_id) = &args.cli.model {
        args.config.write().set_model(model_id)?;
    }
    if args.cli.no_stream {
        args.config.write().stream = false;
    }
    if args.cli.empty_session {
        args.config.write().empty_session()?;
    }
    if args.cli.save_session {
        args.config.write().set_save_session_this_time()?;
    }
    if args.cli.info {
        let _info = args.config.read().info()?;
        println!("{{info}}");
        return Ok(());
    }
    if let Some(addr) = args.cli.serve {
        return serve::run(args.config, addr).await;
    }
    let is_repl = args.config.read().working_mode.is_repl();
    if args.cli.rebuild_rag {
        Config::rebuild_rag(&args.config, abort_signal.clone()).await?;
        if is_repl {
            return Ok(());
        }
    }
    if let Some(_name) = &args.cli.macro_name {
        // macro_execute(&config, name, text.as_deref(), abort_signal.clone()).await?;
        return Ok(());
    }
    if args.cli.execute && !is_repl {
        let input = create_input(crate::main_create_input::CreateInputArgs {
            config: &args.config,
            text: args.text,
            file: &args.cli.file,
            abort_signal: abort_signal.clone(),
        }).await?;
        shell_execute(crate::main_shell_execute::ShellExecuteArgs {
            config: args.config.clone(),
            shell: SHELL.clone(),
            input,
            abort_signal: abort_signal.clone(),
        }).await?;
        return Ok(());
    }
    args.config.write().apply_prelude()?;
    match is_repl {
        false => {
            let mut input = create_input(crate::main_create_input::CreateInputArgs {
                config: &args.config,
                text: args.text,
                file: &args.cli.file,
                abort_signal: abort_signal.clone(),
            }).await?;
            input.use_embeddings(abort_signal.clone()).await?;
            start_directive(crate::main_start_directive::StartDirectiveArgs {
                config: args.config.clone(),
                input,
                code_mode: args.cli.code,
                abort_signal,
            }).await
        }
        true => {
            if !*IS_STDOUT_TERMINAL {
                bail!("No TTY for REPL")
            }
            start_interactive(crate::main_start_interactive::StartInteractiveArgs {
                config: args.config.clone(),
            }).await
        }
    }
}