mod agent;
mod input;
mod role;
mod session;
mod types;

pub use self::agent::{complete_agent_variables, list_agents, Agent, AgentVariables};
pub use self::input::Input;
pub use self::role::{
    Role, RoleLike, CODE_ROLE, CREATE_TITLE_ROLE, EXPLAIN_SHELL_ROLE, SHELL_ROLE,
};
pub use self::types::{WorkingMode, LastMessage, StateFlags, AssertState};
use self::session::Session;

use crate::client::{
    create_client_config, list_client_types, list_models, ClientConfig, MessageContentToolCalls,
    Model, ModelType, ProviderModels, OPENAI_COMPATIBLE_PROVIDERS,
};
use crate::function::{FunctionDeclaration, Functions, ToolResult};
use crate::rag::Rag;
use crate::render::{MarkdownRender, RenderOptions};
use crate::repl::{run_repl_command, split_args_text};
use crate::utils::*;

use anyhow::{anyhow, bail, Context, Result};
use indexmap::IndexMap;
use inquire::{list_option::ListOption, validator::Validation, Confirm, MultiSelect, Select, Text};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::json;
use simplelog::LevelFilter;
use std::collections::{HashMap, HashSet};
use std::{
    env,
    fs::{
        create_dir_all,
        read_dir,
        read_to_string,
        remove_dir_all,
        remove_file,
        File,
        OpenOptions,
    },
    io::Write,
    path::{Path, PathBuf},
    process,
    sync::{Arc, OnceLock},
};
use syntect::highlighting::ThemeSet;
use terminal_colorsaurus::{color_scheme, ColorScheme, QueryOptions};
use crate::auth::oauth_split::oauth_config::OAuthConfig;

pub const TEMP_ROLE_NAME: &str = "%%";
pub const TEMP_RAG_NAME: &str = "temp";
pub const TEMP_SESSION_NAME: &str = "temp";

/// Monokai Extended
const DARK_THEME: &[u8] = include_bytes!("../../assets/monokai-extended.theme.bin");
const LIGHT_THEME: &[u8] = include_bytes!("../../assets/monokai-extended-light.theme.bin");

const CONFIG_FILE_NAME: &str = "config.yaml";
const ROLES_DIR_NAME: &str = "roles";
const MACROS_DIR_NAME: &str = "macros";
const ENV_FILE_NAME: &str = ".env";
const MESSAGES_FILE_NAME: &str = "messages.md";
const SESSIONS_DIR_NAME: &str = "sessions";
const RAGS_DIR_NAME: &str = "rags";
const FUNCTIONS_DIR_NAME: &str = "functions";
const FUNCTIONS_FILE_NAME: &str = "functions.json";
const FUNCTIONS_BIN_DIR_NAME: &str = "bin";
const AGENTS_DIR_NAME: &str = "agents";

const CLIENTS_FIELD: &str = "clients";

const SERVE_ADDR: &str = "127.0.0.1:8000";

const SYNC_MODELS_URL: &str =
    "https://raw.githubusercontent.com/sigoden/aichat/refs/heads/main/models.yaml";

const SUMMARIZE_PROMPT: &str =
    "Summarize the discussion briefly in 200 words or less to use as a prompt for future context.";
const SUMMARY_PROMPT: &str = "This is a summary of the chat history as a recap: ";

const RAG_TEMPLATE: &str = r#"Answer the query based on the context while respecting the rules. (user query, some textual context and rules, all inside xml tags)

<context>
__CONTEXT__
</context>

<rules>
- If you don't know, just say so.
- If you are not sure, ask for clarification.
- Answer in the same language as the user query.
- If the context appears unreadable or of poor quality, tell the user then answer as best as you can.
- If the answer is not in the context but you think you know the answer, explain that to the user then answer with your own knowledge.
- Answer directly and without using xml tags.
</rules>

<user_query>
__INPUT__
</user_query>"#;

const LEFT_PROMPT: &str = "{color.green}{?session {?agent {agent}>}}{session}{?role /}}{!session {?agent {agent}>}}{role}{?rag @{rag}}{color.cyan}{?session )}{!session >}{color.reset} ";
const RIGHT_PROMPT: &str = "{color.purple}{?session {?consume_tokens {consume_tokens}({consume_percent}%)}{!consume_tokens {consume_tokens}}}{color.reset}";

static EDITOR: OnceLock<Option<String>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(rename(serialize = "model", deserialize = "model"))]
    #[serde(default)]
    pub model_id: String,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,

    pub dry_run: bool,
    pub stream: bool,
    pub save: bool,
    pub keybindings: String,
    pub editor: Option<String>,
    pub wrap: Option<String>,
    pub wrap_code: bool,

    pub function_calling: bool,
    pub mapping_tools: IndexMap<String, String>,
    pub use_tools: Option<String>,

    pub repl_prelude: Option<String>,
    pub cmd_prelude: Option<String>,
    pub agent_prelude: Option<String>,

    pub save_session: Option<bool>,
    pub compress_threshold: usize,
    pub summarize_prompt: Option<String>,
    pub summary_prompt: Option<String>,

    pub rag_embedding_model: Option<String>,
    pub rag_reranker_model: Option<String>,
    pub rag_top_k: usize,
    pub rag_chunk_size: Option<usize>,
    pub rag_chunk_overlap: Option<usize>,
    pub rag_template: Option<String>,

    #[serde(default)]
    pub document_loaders: HashMap<String, String>,

    pub highlight: bool,
    pub theme: Option<String>,
    pub left_prompt: Option<String>,
    pub right_prompt: Option<String>,

    pub serve_addr: Option<String>,
    pub user_agent: Option<String>,
    pub save_shell_history: bool,
    pub sync_models_url: Option<String>,

    pub clients: Vec<ClientConfig>,

    pub oauth: OAuthConfig, // New field

    

    #[serde(skip)]
    pub macro_flag: bool,
    #[serde(skip)]
    pub info_flag: bool,
    #[serde(skip)]
    pub agent_variables: Option<AgentVariables>,

    #[serde(skip)]
    pub model: Model,
    #[serde(skip)]
    pub functions: Functions,
    #[serde(skip)]
    pub working_mode: WorkingMode,
    #[serde(skip)]
    pub last_message: Option<LastMessage>,

    #[serde(skip)]
    pub role: Option<Role>,
    #[serde(skip)]
    pub session: Option<Session>,
    #[serde(skip)]
    pub rag: Option<Arc<Rag>>,
    #[serde(skip)]
    pub agent: Option<Agent>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_id: Default::default(),
            temperature: None,
            top_p: None,

            dry_run: false,
            stream: true,
            save: false,
            keybindings: "emacs".into(),
            editor: None,
            wrap: None,
            wrap_code: false,

            function_calling: true,
            mapping_tools: Default::default(),
            use_tools: None,

            repl_prelude: None,
            cmd_prelude: None,
            agent_prelude: None,

            save_session: None,
            compress_threshold: 4000,
            summarize_prompt: None,
            summary_prompt: None,

            rag_embedding_model: None,
            rag_reranker_model: None,
            rag_top_k: 5,
            rag_chunk_size: None,
            rag_chunk_overlap: None,
            rag_template: None,

            document_loaders: Default::default(),

            highlight: true,
            theme: None,
            left_prompt: None,
            right_prompt: None,

            serve_addr: None,
            user_agent: None,
            save_shell_history: true,
            sync_models_url: None,

            clients: vec![],

            oauth: Default::default(), // New field

            macro_flag: false,
            info_flag: false,
            agent_variables: None,

            model: Default::default(),
            functions: Default::default(),
            working_mode: WorkingMode::Cmd,
            last_message: None,

            role: None,
            session: None,
            rag: None,
            agent: None,
        }
    }
}

pub type GlobalConfig = Arc<RwLock<Config>>;

impl Config {
    pub async fn init(working_mode: WorkingMode, info_flag: bool) -> Result<Self> {
        let mut config = Config::default();
        config.working_mode = working_mode;
        config.info_flag = info_flag;
        Ok(config)
    }

    pub fn config_dir() -> PathBuf {
        if let Ok(v) = env::var(get_env_name("config_dir")) {
            PathBuf::from(v)
        } else if let Ok(v) = env::var("XDG_CONFIG_HOME") {
            PathBuf::from(v).join(env!("CARGO_CRATE_NAME"))
        } else {
            let dir = dirs::config_dir().expect("No user's config directory");
            dir.join(env!("CARGO_CRATE_NAME"))
        }
    }

    pub fn local_path(name: &str) -> PathBuf {
        Self::config_dir().join(name)
    }

    pub fn config_file() -> PathBuf {
        match env::var(get_env_name("config_file")) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::local_path(CONFIG_FILE_NAME),
        }
    }

    pub fn roles_dir() -> PathBuf {
        match env::var(get_env_name("roles_dir")) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::local_path(ROLES_DIR_NAME),
        }
    }

    pub fn role_file(name: &str) -> PathBuf {
        Self::roles_dir().join(format!("{name}.md"))
    }

    pub fn macros_dir() -> PathBuf {
        match env::var(get_env_name("macros_dir")) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::local_path(MACROS_DIR_NAME),
        }
    }

    pub fn macro_file(name: &str) -> PathBuf {
        Self::macros_dir().join(format!("{name}.yaml"))
    }

    pub fn env_file() -> PathBuf {
        match env::var(get_env_name("env_file")) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::local_path(ENV_FILE_NAME),
        }
    }

    pub fn messages_file(&self) -> PathBuf {
        match &self.agent {
            None => match env::var(get_env_name("messages_file")) {
                Ok(value) => PathBuf::from(value),
                Err(_) => Self::local_path(MESSAGES_FILE_NAME),
            },
            Some(agent) => Self::agent_data_dir(agent.name()).join(MESSAGES_FILE_NAME),
        }
    }

    pub fn sessions_dir(&self) -> PathBuf {
        match &self.agent {
            None => match env::var(get_env_name("sessions_dir")) {
                Ok(value) => PathBuf::from(value),
                Err(_) => Self::local_path(SESSIONS_DIR_NAME),
            },
            Some(agent) => Self::agent_data_dir(agent.name()).join(SESSIONS_DIR_NAME),
        }
    }

    pub fn rags_dir() -> PathBuf {
        match env::var(get_env_name("rags_dir")) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::local_path(RAGS_DIR_NAME),
        }
    }

    pub fn functions_dir() -> PathBuf {
        match env::var(get_env_name("functions_dir")) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::local_path(FUNCTIONS_DIR_NAME),
        }
    }

    pub fn functions_file() -> PathBuf {
        Self::functions_dir().join(FUNCTIONS_FILE_NAME)
    }

    pub fn functions_bin_dir() -> PathBuf {
        Self::functions_dir().join(FUNCTIONS_BIN_DIR_NAME)
    }

    pub fn session_file(&self, name: &str) -> PathBuf {
        match name.split_once("/") {
            Some((dir, name)) => self.sessions_dir().join(dir).join(format!("{name}.yaml")),
            None => self.sessions_dir().join(format!("{name}.yaml")),
        }
    }

    pub fn rag_file(&self, name: &str) -> PathBuf {
        match &self.agent {
            Some(agent) => Self::agent_rag_file(agent.name(), name),
            None => Self::rags_dir().join(format!("{name}.yaml")),
        }
    }

    pub fn agents_data_dir() -> PathBuf {
        Self::local_path(AGENTS_DIR_NAME)
    }

    pub fn agent_data_dir(name: &str) -> PathBuf {
        match env::var(format!("{}_DATA_DIR", normalize_env_name(name))) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::agents_data_dir().join(name),
        }
    }

    pub fn agent_config_file(name: &str) -> PathBuf {
        match env::var(format!("{}_CONFIG_FILE", normalize_env_name(name))) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::agent_data_dir(name).join(CONFIG_FILE_NAME),
        }
    }

    pub fn agent_rag_file(agent_name: &str, rag_name: &str) -> PathBuf {
        Self::agent_data_dir(agent_name).join(format!("{rag_name}.yaml"))
    }

    pub fn agents_functions_dir() -> PathBuf {
        Self::functions_dir().join(AGENTS_DIR_NAME)
    }

    pub fn agent_functions_dir(name: &str) -> PathBuf {
        match env::var(format!("{}_FUNCTIONS_DIR", normalize_env_name(name))) {
            Ok(value) => PathBuf::from(value),
            Err(_) => Self::agents_functions_dir().join(name),
        }
    }

    pub fn models_override_file() -> PathBuf {
        Self::local_path("models-override.yaml")
    }

    pub fn state(&self) -> StateFlags {
        let mut flags = StateFlags::empty();
        if let Some(session) = &self.session {
            if session.is_empty() {
                flags |= StateFlags::SESSION_EMPTY;
            } else {
                flags |= StateFlags::SESSION;
            }
            if session.role_name().is_some() {
                flags |= StateFlags::ROLE;
            }
        } else if self.role.is_some() {
            flags |= StateFlags::ROLE;
        }
        if self.agent.is_some() {
            flags |= StateFlags::AGENT;
        }
        if self.rag.is_some() {
            flags |= StateFlags::RAG;
        }
        flags
    }

    pub fn serve_addr(&self) -> String {
        self.serve_addr.clone().unwrap_or_else(|| SERVE_ADDR.into())
    }

    pub fn log_config(is_serve: bool) -> Result<(LevelFilter, Option<PathBuf>)> {
        let log_level = env::var(get_env_name("log_level"))
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(match cfg!(debug_assertions) {
                true => LevelFilter::Debug,
                false => {
                    if is_serve {
                        LevelFilter::Info
                    } else {
                        LevelFilter::Off
                    }
                }
            });
        if log_level == LevelFilter::Off {
            return Ok((log_level, None));
        }
        let log_path = match env::var(get_env_name("log_path")) {
            Ok(v) => Some(PathBuf::from(v)),
            Err(_) => match is_serve {
                true => None,
                false => Some(Config::local_path(&format!(
                    "{}.log",
                    env!("CARGO_CRATE_NAME")
                ))),
            },
        };
        Ok((log_level, log_path))
    }

    pub fn edit_config(&self) -> Result<()> {
        let config_path = Self::config_file();
        let editor = self.editor.clone().unwrap_or_else(|| "vi".to_string()); // Provide a default editor if none is configured
        edit_file(&editor, &config_path)?;
        println!(
            "NOTE: Remember to restart {} if there are changes made to '{}'",
            env!("CARGO_CRATE_NAME"),
            config_path.display(),
        );
        Ok(())
    }

    pub fn current_model(&self) -> &Model {
        if let Some(session) = self.session.as_ref() {
            session.model()
        } else if let Some(agent) = self.agent.as_ref() {
            agent.model()
        } else if let Some(role) = self.role.as_ref() {
            role.model()
        } else {
            &self.model
        }
    }

    pub fn role_like_mut(&mut self) -> Option<&mut dyn RoleLike> {
        if let Some(session) = self.session.as_mut() {
            Some(session)
        } else if let Some(agent) = self.agent.as_mut() {
            Some(agent)
        } else if let Some(role) = self.role.as_mut() {
            Some(role)
        } else {
            None
        }
    }

    pub fn extract_role(&self) -> Role {
        if let Some(session) = self.session.as_ref() {
            session.to_role()
        } else if let Some(agent) = self.agent.as_ref() {
            agent.to_role()
        } else if let Some(role) = self.role.as_ref() {
            role.clone()
        } else {
            let mut role = Role::default();
            role.batch_set(
                &self.model,
                self.temperature,
                self.top_p,
                self.use_tools.clone(),
            );
            role
        }
    }

    pub fn info(&self) -> Result<String> {
        if let Some(agent) = &self.agent {
            let output = agent.export()?;
            if let Some(session) = &self.session {
                let session = session
                    .export()? 
                    .split('\n')
                    .map(|v| format!("  {v}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(format!("{output}session:\n{session}"))
            } else {
                Ok(output)
            }
        } else if let Some(session) = &self.session {
            session.export()
        } else if let Some(role) = &self.role {
            Ok(role.export())
        } else if let Some(rag) = &self.rag {
            rag.export()
        } else {
            self.sysinfo()
        }
    }

    pub fn sysinfo(&self) -> Result<String> {
        let display_path = |path: &Path| path.display().to_string();
        let wrap = self
            .wrap
            .clone()
            .map_or_else(|| String::from("no"), |v| v.to_string());
        let (rag_reranker_model, rag_top_k) = match &self.rag {
            Some(rag) => rag.get_config(),
            None => (self.rag_reranker_model.clone(), self.rag_top_k),
        };
        let role = self.extract_role();
        let mut items = vec![
            ("model", role.model().id()),
            // ("temperature", format_option_value(&role.temperature())),
            // ("top_p", format_option_value(&role.top_p())),
            // ("use_tools", format_option_value(&role.use_tools())),
            (
                "max_output_tokens",
                role.model()
                    .max_tokens_param()
                    .map(|v| format!("{v} (current model)"))
                    .unwrap_or_else(|| "null".into()),
            ),
            // ("save_session", format_option_value(&self.save_session)),
            ("compress_threshold", self.compress_threshold.to_string()),
            // ("rag_reranker_model",
            //     format_option_value(&rag_reranker_model),
            // ),
            ("rag_top_k", rag_top_k.to_string()),
            ("dry_run", self.dry_run.to_string()),
            ("function_calling", self.function_calling.to_string()),
            ("stream", self.stream.to_string()),
            ("save", self.save.to_string()),
            ("keybindings", self.keybindings.clone()),
            ("wrap", wrap),
            ("wrap_code", self.wrap_code.to_string()),
            ("highlight", self.highlight.to_string()),
            // ("theme", format_option_value(&self.theme)),
            ("config_file", display_path(&Self::config_file())),
            ("env_file", display_path(&Self::env_file())),
            ("roles_dir", display_path(&Self::roles_dir())),
            ("sessions_dir", display_path(&self.sessions_dir())),
            ("rags_dir", display_path(&Self::rags_dir())),
            ("macros_dir", display_path(&Self::macros_dir())),
            ("functions_dir", display_path(&Self::functions_dir())),
            ("messages_file", display_path(&self.messages_file())),
        ];
        if let Ok((_, Some(log_path))) = Self::log_config(self.working_mode.is_serve()) {
            items.push(("log_path", display_path(&log_path)));
        }
        let output = items
            .iter()
            .map(|(name, value)| format!("{name:<24}{value}\n"))
            .collect::<Vec<String>>()
            .join("");
        Ok(output)
    }

    pub fn render_options(&self) -> Result<RenderOptions> {
        use std::io::Cursor;

        let theme = if let Some(theme) = self.theme.clone() {
            let theme_str = read_to_string(Self::local_path(&format!("themes/{theme}.tmTheme")))?;
            Some(ThemeSet::load_from_reader(&mut Cursor::new(theme_str.as_bytes()))?)
        } else {
            let theme_bytes = if self.light_theme() {
                LIGHT_THEME
            } else {
                DARK_THEME
            };
            Some(ThemeSet::load_from_reader(&mut Cursor::new(theme_bytes.as_ref()))?)
        };

        let wrap = self.wrap.clone();

        Ok(RenderOptions {
            wrap,
            wrap_code: self.wrap_code,
            theme,
            truecolor: true,
        })
    }

    pub fn light_theme(&self) -> bool {
        if let Some(theme) = &self.theme {
            theme.ends_with("-light")
        } else {
            false
        }
    }

    pub fn update(config: &GlobalConfig, data: &str) -> Result<()> {
        let parts: Vec<&str> = data.split_whitespace().collect();
        if parts.len() != 2 {
            bail!("Usage: .set <key> <value>. If value is null, unset key.");
        }
        let key = parts[0];
        let value = parts[1];
        match key {
            "temperature" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.parse().with_context(|| "Invalid value")?)
                };
                config.write().set_temperature(value);
            }
            "top_p" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.parse().with_context(|| "Invalid value")?)
                };
                config.write().set_top_p(value);
            }
            "use_tools" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.to_string())
                };
                config.write().set_use_tools(value);
            }
            "max_output_tokens" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.parse().with_context(|| "Invalid value")?)
                };
                config.write().set_max_output_tokens(value);
            }
            "save_session" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.parse().with_context(|| "Invalid value")?)
                };
                config.write().set_save_session(value);
            }
            "compress_threshold" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.parse().with_context(|| "Invalid value")?)
                };
                config.write().set_compress_threshold(value);
            }
            "rag_reranker_model" => {
                let value = if value == "null" {
                    None
                } else {
                    Some(value.to_string())
                };
                Self::set_rag_reranker_model(config, value)?;
            }
            "rag_top_k" => {
                let value = value.parse().with_context(|| "Invalid value")?;
                Self::set_rag_top_k(config, value)?;
            }
            "dry_run" => {
                let value = value.parse().with_context(|| "Invalid value")?;
                config.write().dry_run = value;
            }
            "function_calling" => {
                let value = value.parse().with_context(|| "Invalid value")?;
                if value && config.write().functions.is_empty() {
                    bail!("Function calling cannot be enabled because no functions are installed.")
                }
                config.write().function_calling = value;
            }
            "stream" => {
                let value = value.parse().with_context(|| "Invalid value")?;
                config.write().stream = value;
            }
            "save" => {
                let value = value.parse().with_context(|| "Invalid value")?;
                config.write().save = value;
            }
            "highlight" => {
                let value = value.parse().with_context(|| "Invalid value")?;
                config.write().highlight = value;
            }
            _ => bail!("Unknown key '{key}'"),
        }
        Ok(())
    }

    pub fn delete(config: &GlobalConfig, kind: &str) -> Result<()> {
        let (dir, file_ext) = match kind {
            "role" => (Self::roles_dir(), Some(".md")),
            "session" => (config.read().sessions_dir(), Some(".yaml")),
            "rag" => (Self::rags_dir(), Some(".yaml")),
            "macro" => (Self::macros_dir(), Some(".yaml")),
            "agent-data" => (Self::agents_data_dir(), None),
            _ => bail!("Unknown kind '{kind}'"),
        };
        let names = match read_dir(&dir) {
            Ok(rd) => {
                let mut names = vec![];
                for entry in rd.flatten() {
                    let name = entry.file_name();
                    match file_ext {
                        Some(file_ext) => {
                            if let Some(name) = name.to_string_lossy().strip_suffix(file_ext) {
                                names.push(name.to_string());
                            }
                        }
                        None => {
                            if entry.path().is_dir() {
                                names.push(name.to_string_lossy().to_string());
                            }
                        }
                    }
                }
                names.sort_unstable();
                names
            }
            Err(_) => vec![],
        };

        if names.is_empty() {
            bail!("No {kind} to delete")
        }

        let select_names = MultiSelect::new(&format!("Select {kind} to delete:"), names)
            .with_validator(|list: &[ListOption<&String>]| {
                if list.is_empty() {
                    Ok(Validation::Invalid(
                        "At least one item must be selected".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()?;

        for name in select_names {
            match file_ext {
                Some(ext) => {
                    let path = dir.join(format!("{name}{ext}"));
                    remove_file(&path).with_context(|| {
                        format!("Failed to delete {kind} at '{}'", path.display())
                    })?;
                }
                None => {
                    let path = dir.join(name);
                    remove_dir_all(&path).with_context(|| {
                        format!("Failed to delete {kind} at '{}'", path.display())
                    })?;
                }
            }
        }
        println!("✓ Successfully deleted {kind}.");
        Ok(())
    }

    pub fn set_temperature(&mut self, value: Option<f64>) {
        match self.role_like_mut() {
            Some(role_like) => role_like.set_temperature(value),
            None => self.temperature = value,
        }
    }

    pub fn set_top_p(&mut self, value: Option<f64>) {
        match self.role_like_mut() {
            Some(role_like) => role_like.set_top_p(value),
            None => self.top_p = value,
        }
    }

    pub fn set_use_tools(&mut self, value: Option<String>) {
        match self.role_like_mut() {
            Some(role_like) => role_like.set_use_tools(value),
            None => self.use_tools = value,
        }
    }

    pub fn set_save_session(&mut self, value: Option<bool>) {
        if let Some(session) = self.session.as_mut() {
            session.set_save_session(value);
        } else {
            self.save_session = value;
        }
    }

    pub fn set_compress_threshold(&mut self, value: Option<usize>) {
        if let Some(session) = self.session.as_mut() {
            session.set_compress_threshold(value);
        } else {
            self.compress_threshold = value.unwrap_or_default();
        }
    }

    pub fn set_rag_reranker_model(config: &GlobalConfig, value: Option<String>) -> Result<()> {
        if let Some(id) = &value {
            Model::retrieve_model(&config.read(), id, ModelType::Reranker)?;
        }
        let has_rag = config.read().rag.is_some();
        match has_rag {
            true => { /* update_rag(config, |rag| { rag.set_reranker_model(value)?; Ok(()) })?; */ }
            false => config.write().rag_reranker_model = value,
        }
        Ok(())
    }

    pub fn set_rag_top_k(config: &GlobalConfig, value: usize) -> Result<()> {
        let has_rag = config.read().rag.is_some();
        match has_rag {
            true => { /* update_rag(config, |rag| { rag.set_top_k(value)?; Ok(()) })?; */ }
            false => config.write().rag_top_k = value,
        }
        Ok(())
    }

    pub fn set_wrap(&mut self, value: &str) -> Result<()> {
        if value == "no" {
            self.wrap = None;
        } else if value == "auto" {
            self.wrap = Some(value.into());
        } else {
            value
                .parse::<u16>()
                .map_err(|_| anyhow!("Invalid wrap value"))?;
            self.wrap = Some(value.into())
        }
        Ok(())
    }

    pub fn set_max_output_tokens(&mut self, value: Option<isize>) {
        match self.role_like_mut() {
            Some(role_like) => {
                let mut model = role_like.model().clone();
                model.set_max_tokens(value, true);
                role_like.set_model(model);
            }
            None => {
                self.model.set_max_tokens(value, true);
            }
        };
    }

    pub fn set_model(&mut self, model_id: &str) -> Result<()> {
        let model = Model::retrieve_model(self, model_id, ModelType::Chat)?;
        match self.role_like_mut() {
            Some(role_like) => role_like.set_model(model),
            None => {
                self.model = model;
            }
        }
        Ok(())
    }

    pub fn use_prompt(&mut self, prompt: &str) -> Result<()> {
        let mut role = Role::new(TEMP_ROLE_NAME, prompt);
        role.set_model(self.current_model().clone());
        self.use_role_obj(role)
    }

    pub fn use_role(&mut self, name: &str) -> Result<()> {
        let role = self.retrieve_role(name)?;
        self.use_role_obj(role)
    }

    pub fn use_role_obj(&mut self, role: Role) -> Result<()> {
        if self.agent.is_some() {
            bail!("Cannot perform this operation because you are using a agent")
        }
        if let Some(session) = self.session.as_mut() {
            session.guard_empty()?;
            session.set_role(role);
        } else {
            self.role = Some(role);
        }
        Ok(())
    }

    pub fn role_info(&self) -> Result<String> {
        if let Some(session) = &self.session {
            if session.role_name().is_some() {
                let role = session.to_role();
                Ok(role.export())
            } else {
                bail!("No session role")
            }
        } else if let Some(role) = &self.role {
            Ok(role.export())
        } else {
            bail!("No role")
        }
    }

    pub fn exit_role(&mut self) -> Result<()> {
        if let Some(session) = self.session.as_mut() {
            session.guard_empty()?;
            session.clear_role();
        } else if self.role.is_some() {
            self.role = None;
        }
        Ok(())
    }

    pub fn retrieve_role(&self, name: &str) -> Result<Role> {
        let names = Self::list_roles(false);
        let mut role = if names.contains(&name.to_string()) {
            let path = Self::role_file(name);
            let content = read_to_string(&path)?;
            Role::new(name, &content)
        } else {
            Role::builtin(name)?
        };
        let current_model = self.current_model().clone();
        match role.model_id() {
            Some(model_id) => {
                if current_model.id() != model_id {
                    let model = Model::retrieve_model(self, model_id, ModelType::Chat)?;
                    role.set_model(model);
                } else {
                    role.set_model(current_model);
                }
            }
            None => {
                role.set_model(current_model);
                if role.temperature().is_none() {
                    role.set_temperature(self.temperature);
                }
                if role.top_p().is_none() {
                    role.set_top_p(self.top_p);
                }
            }
        }
        Ok(role)
    }

    pub fn new_role(&mut self, name: &str) -> Result<()> {
        if self.macro_flag {
            bail!("No role");
        }
        let ans = Confirm::new("Create a new role?")
            .with_default(true)
            .prompt()?;
        if ans {
            self.upsert_role(name)?;
        } else {
            bail!("No role");
        }
        Ok(())
    }

    pub fn edit_role(&mut self) -> Result<()> {
        let role_name;
        if let Some(session) = self.session.as_ref() {
            if let Some(name) = session.role_name().map(|v| v.to_string()) {
                if session.is_empty() {
                    role_name = Some(name);
                } else {
                    bail!("Cannot perform this operation because you are in a non-empty session")
                }
            } else {
                bail!("No role")
            }
        } else {
            role_name = self.role.as_ref().map(|v| v.name().to_string());
        }
        let name = role_name.ok_or_else(|| anyhow!("No role"))?;
        self.upsert_role(&name)?;
        self.use_role(&name)
    }

    pub fn upsert_role(&mut self, name: &str) -> Result<()> {
        let role_path = Self::role_file(name);
        // ensure_parent_exists(&role_path)?;
        let editor = self.editor.clone().unwrap_or_else(|| "vi".to_string()); // Provide a default editor if none is configured
        edit_file(&editor, &role_path)?;
        if self.working_mode.is_repl() {
            println!("✓ Saved the role to '{}'.", role_path.display());
        }
        Ok(())
    }

    pub fn save_role(&mut self, name: Option<&str>) -> Result<()> {
        let mut role_name = match &self.role {
            Some(role) => {
                if role.has_args() {
                    bail!("Unable to save the role with arguments (whose name contains '#')")
                }
                match name {
                    Some(v) => v.to_string(),
                    None => role.name().to_string(),
                }
            }
            None => bail!("No role"),
        };
        if role_name == TEMP_ROLE_NAME {
            role_name = Text::new("Role name:")
                .with_validator(|input: &str| {
                    let input = input.trim();
                    if input.is_empty() {
                        Ok(Validation::Invalid("This name is required".into()))
                    } else if input == TEMP_ROLE_NAME {
                        Ok(Validation::Invalid("This name is reserved".into()))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?;
        }
        let role_path = Self::role_file(&role_name);
        if let Some(role) = self.role.as_mut() {
            role.save(&role_name, &role_path, self.working_mode.is_repl())?;
        }

        Ok(())
    }

    pub fn all_roles() -> Vec<Role> {
        let mut roles: HashMap<String, Role> = Role::list_builtin_roles()
            .iter()
            .map(|v| (v.name().to_string(), v.clone()))
            .collect();
        let names = Self::list_roles(false);
        for name in names {
            if let Ok(content) = read_to_string(Self::role_file(&name)) {
                let role = Role::new(&name, &content);
                roles.insert(name, role);
            }
        }
        let mut roles: Vec<_> = roles.into_values().collect();
        roles.sort_unstable_by(|a, b| a.name().cmp(b.name()));
        roles
    }

    pub fn list_roles(with_builtin: bool) -> Vec<String> {
        let mut names = HashSet::new();
        if let Ok(rd) = read_dir(Self::roles_dir()) {
            for entry in rd.flatten() {
                if let Some(name) = entry
                    .file_name()
                    .to_str()
                    .and_then(|v| v.strip_suffix(".md"))
                {
                    names.insert(name.to_string());
                }
            }
        }
        if with_builtin {
            names.extend(Role::list_builtin_role_names());
        }
        let mut names: Vec<_> = names.into_iter().collect();
        names.sort_unstable();
        names
    }

    pub fn has_role(name: &str) -> bool {
        let names = Self::list_roles(true);
        names.contains(&name.to_string())
    }

    pub fn use_session(&mut self, session_name: Option<&str>) -> Result<()> {
        if self.session.is_some() {
            bail!(
                "Already in a session, please run '.exit session' first to exit the current session."
            );
        }
        let mut session;
        match session_name {
            None | Some(TEMP_SESSION_NAME) => {
                let session_file = self.session_file(TEMP_SESSION_NAME);
                if session_file.exists() {
                    remove_file(session_file).with_context(|| {
                        format!("Failed to cleanup previous '{}' session", TEMP_SESSION_NAME)
                    })?;
                }
                session = Some(Session::new(self, TEMP_SESSION_NAME));
            }
            Some(name) => {
                let session_path = self.session_file(name);
                if !session_path.exists() {
                    session = Some(Session::new(self, name));
                } else {
                    session = Some(Session::load(self, name, &session_path)?);
                }
            }
        }
        let mut new_session = false;
        if let Some(session) = session.as_mut() {
            if session.is_empty() {
                new_session = true;
                if let Some(LastMessage { input, output, continuous })
                    = &self.last_message
                {
                    if (*continuous && !output.is_empty()) && self.agent.is_some() == input.with_agent() {
                        let ans = Confirm::new(
                            "Start a session that incorporates the last question and answer?",
                        )
                        .with_default(false)
                        .prompt()?;
                        if ans {
                            session.add_message(input, output)?;
                        }
                    }
                }
            }
        }
        self.session = session;
        self.init_agent_session_variables(new_session)?;
        Ok(())
    }

    pub fn session_info(&self) -> Result<String> {
        if let Some(session) = &self.session {
            let render_options = self.render_options()?;
            let mut markdown_render = MarkdownRender::init(render_options)?;
            let agent_info: Option<(String, Vec<String>)> = self.agent.as_ref().map(|agent| {
                let functions = agent
                    .functions()
                    .declarations()
                    .iter()
                    .filter_map(|v| if v.agent { Some(v.name.clone()) } else { None })
                    .collect();
                (agent.name().to_string(), functions)
            });
            session.render(&mut markdown_render, &agent_info)
        } else {
            bail!("No session")
        }
    }

    pub fn exit_session(&mut self) -> Result<()> {
        if let Some(mut session) = self.session.take() {
            let sessions_dir = self.sessions_dir();
            session.exit(&sessions_dir, self.working_mode.is_repl())?;
            self.discontinuous_last_message();
        }
        Ok(())
    }

    pub fn save_session(&mut self, name: Option<&str>) -> Result<()> {
        let session_name = match &self.session {
            Some(session) => match name {
                Some(v) => v.to_string(),
                None => session
                    .autoname()
                    .unwrap_or_else(|| session.name())
                    .to_string(),
            },
            None => bail!("No session"),
        };
        let session_path = self.session_file(&session_name);
        if let Some(session) = self.session.as_mut() {
            session.save(&session_name, &session_path, self.working_mode.is_repl())?;
        }
        Ok(())
    }

    pub fn edit_session(&mut self) -> Result<()> {
        let name = match &self.session {
            Some(session) => session.name().to_string(),
            None => bail!("No session"),
        };
        let session_path = self.session_file(&name);
        self.save_session(Some(&name))?;
        let editor = self.editor.clone().unwrap_or_else(|| "vi".to_string()); // Provide a default editor if none is configured
        edit_file(&editor, &session_path).with_context(|| {
            format!("Failed to edit '{}' with '{}'", session_path.display(), editor)
        })?;
        self.session = Some(Session::load(self, &name, &session_path)?);
        // self.discontinuous_last_message();
        Ok(())
    }

    pub fn empty_session(&mut self) -> Result<()> {
        if let Some(session) = self.session.as_mut() {
            if let Some(agent) = self.agent.as_ref() {
                session.sync_agent(agent);
            }
            session.clear_messages();
        } else {
            bail!("No session")
        }
        // self.discontinuous_last_message();
        Ok(())
    }

    pub fn set_save_session_this_time(&mut self) -> Result<()> {
        if let Some(session) = self.session.as_mut() {
            session.set_save_session_this_time();
        } else {
            bail!("No session")
        }
        Ok(())
    }

    pub fn list_sessions(&self) -> Vec<String> {
        list_file_names(self.sessions_dir(), ".yaml")
    }

    pub fn list_autoname_sessions(&self) -> Vec<String> {
        list_file_names(self.sessions_dir().join("_"), ".yaml")
    }

    pub fn maybe_compress_session(config: GlobalConfig) {
        let mut need_compress = false;
        {
            let mut config = config.write();
            let compress_threshold = config.compress_threshold;
            if let Some(session) = config.session.as_mut() {
                if session.need_compress(compress_threshold) {
                    session.set_compressing(true);
                    need_compress = true;
                }
            }
        };
        if !need_compress {
            return;
        }
        let color = if config.read().light_theme() {
            nu_ansi_term::Color::LightGray
        } else {
            nu_ansi_term::Color::DarkGray
        };
        print!(
            "\n📢 {}\n",
            color.italic().paint("Compressing the session."),
        );
        tokio::spawn(async move {
            if let Err(err) = Config::compress_session(&config).await {
                warn!("Failed to compress the session: {err}");
            }
            if let Some(session) = config.write().session.as_mut() {
                session.set_compressing(false);
            }
        });
    }

    pub async fn compress_session(config: &GlobalConfig) -> Result<()> {
        match config.read().session.as_ref() {
            Some(session) => {
                if !session.has_user_messages() {
                    bail!("No need to compress since there are no messages in the session")
                }
            }
            None => bail!("No session"),
        }

        let prompt = config
            .read()
            .summarize_prompt
            .clone()
            .unwrap_or_else(|| SUMMARIZE_PROMPT.into());
        let input = Input::from_str(config, &prompt, None);
        let summary = input.fetch_chat_text().await?;
        let summary_prompt = config
            .read()
            .summary_prompt
            .clone()
            .unwrap_or_else(|| SUMMARY_PROMPT.into());
        if let Some(session) = config.write().session.as_mut() {
            session.compress(format!("{summary_prompt}{summary}"));
        }
        // config.write().discontinuous_last_message();
        Ok(())
    }

    pub fn is_compressing_session(&self) -> bool {
        self.session
            .as_ref()
            .map(|v| v.compressing())
            .unwrap_or_default()
    }

    pub fn maybe_autoname_session(config: GlobalConfig) {
        let mut need_autoname = false;
        if let Some(session) = config.write().session.as_mut() {
            if session.need_autoname() {
                session.set_autonaming(true);
                need_autoname = true;
            }
        }
        if !need_autoname {
            return;
        }
        let color = if config.read().light_theme() {
            nu_ansi_term::Color::LightGray
        } else {
            nu_ansi_term::Color::DarkGray
        };
        print!("\n📢 {}\n", color.italic().paint("Autonaming the session."),);
        tokio::spawn(async move {
            if let Err(err) = Config::autoname_session(&config).await {
                warn!("Failed to autonaming the session: {err}");
            }
            if let Some(session) = config.write().session.as_mut() {
                session.set_autonaming(false);
            }
        });
    }

    pub async fn autoname_session(config: &GlobalConfig) -> Result<()> {
        let text = match config
            .read()
            .session
            .as_ref()
            .and_then(|v| v.chat_history_for_autonaming())
        {
            Some(v) => v,
            None => bail!("No chat history"),
        };
        let role = config.read().retrieve_role(CREATE_TITLE_ROLE)?;
        let input = Input::from_str(config, &text, Some(role));
        let text = input.fetch_chat_text().await?;
        if let Some(session) = config.write().session.as_mut() {
            session.set_autoname(&text);
        }
        Ok(())
    }

    pub async fn use_rag(
        config: &GlobalConfig,
        rag: Option<&str>,
        abort_signal: AbortSignal,
    ) -> Result<()> {
        if config.read().agent.is_some() {
            bail!("Cannot perform this operation because you are using a agent")
        }
        let rag = match rag {
            None => {
                let rag_path = config.read().rag_file(TEMP_RAG_NAME);
                if rag_path.exists() {
                    remove_file(&rag_path).with_context(|| {
                        format!("Failed to cleanup previous '{}' rag", TEMP_RAG_NAME)
                    })?;
                }
                Rag::init(config, TEMP_RAG_NAME, &rag_path, &[], abort_signal).await? 
            }
            Some(name) => {
                let rag_path = config.read().rag_file(name);
                if !rag_path.exists() {
                    if config.read().working_mode.is_cmd() {
                        bail!("Unknown RAG '{name}'")
                    }
                    Rag::init(config, name, &rag_path, &[], abort_signal).await? 
                } else {
                    Rag::load(config, name, &rag_path)?
                }
            }
        };
        config.write().rag = Some(Arc::new(rag));
        Ok(())
    }

    pub async fn edit_rag_docs(config: &GlobalConfig, abort_signal: AbortSignal) -> Result<()> {
        let mut rag = match config.read().rag.clone() {
            Some(v) => v.as_ref().clone(),
            None => bail!("No RAG"),
        };

        let document_paths = rag.document_paths();
        let temp_file = temp_file(&format!("-rag-{}", rag.name()), ".txt");
        Ok(())
    }

    // Dummy implementations for missing methods/functions
    pub async fn search_rag(
        _config: &GlobalConfig,
        _rag: &Rag,
        _input: &str,
        _abort_signal: AbortSignal,
    ) -> Result<String> {
        Ok("".to_string())
    }

    pub async fn use_agent(
        _config: &GlobalConfig,
        _agent_name: &str,
        _session_name: &str,
        _abort_signal: AbortSignal,
    ) -> Result<()> {
        Ok(())
    }

    pub fn agent_banner(&self) -> Result<String> {
        Ok("".to_string())
    }

    pub async fn rebuild_rag(_config: &GlobalConfig, _abort_signal: AbortSignal) -> Result<()> {
        Ok(())
    }

    pub fn rag_sources(_config: &GlobalConfig) -> Result<String> {
        Ok("".to_string())
    }

    pub fn has_macro(_name: &str) -> bool {
        false
    }

    pub fn new_macro(&mut self, _name: &str) -> Result<()> {
        Ok(())
    }

    pub fn exit_rag(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn exit_agent(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn sync_models(_url: &str, _abort_signal: AbortSignal) -> Result<()> {
        Ok(())
    }

    pub fn list_rags() -> Vec<String> {
        vec![]
    }

    pub fn list_macros() -> Vec<String> {
        vec![]
    }

    pub fn apply_prelude(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn load_models_override() -> Option<IndexMap<String, Model>> {
        None
    }

    pub fn select_functions(&self, _role: &Role) -> Option<Vec<serde_json::Value>> {
        None
    }

    pub fn init_agent_session_variables(&mut self, _new_session: bool) -> Result<()> {
        Ok(())
    }

    pub fn discontinuous_last_message(&mut self) {
        // Do nothing
    }

    pub fn repl_complete(&self, _cmd: &str, _args: &[String], _args_line: &str) -> Vec<(String, String)> {
        vec![]
    }

    pub fn render_prompt_left(&self) -> String {
        "".to_string()
    }

    pub fn render_prompt_right(&self) -> String {
        "".to_string()
    }

    pub fn before_chat_completion(&mut self, _input: &Input) -> Result<()> {
        Ok(())
    }

    pub fn after_chat_completion(&mut self, _input: &Input, _output: &str, _tool_results: &[ToolResult]) -> Result<()> {
        Ok(())
    }
}
