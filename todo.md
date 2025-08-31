# Todo List for aichat

1. include ~/storage/github/gemini-cli
be able to read in ~/.gemini/oauth_creds.json, generate similar credentials and have the user login. 
review this code relative to ~/storage/github/

./ragit/vendor/amazon-q-developer-cli/crates/gemini-auth/Cargo.toml
./ragit/vendor/ask_gemini/Cargo.toml
./gemini-cli/crates/solfunmeme-banner/Cargo.toml
./solfunmeme-dioxus/crates/gemini_cli_lib/Cargo.toml
./libminizinc/crates/gemini_utils/Cargo.toml
./libminizinc/crates/gemini_utils_test/Cargo.toml
./libminizinc/crates/gemini_cli_manager/Cargo.toml
./cargo-to-mcp/vendor/gemini-cli/crates/solfunmeme-banner/Cargo.toml
./gemini_logs/cli/crates/solfunmeme-banner/Cargo.toml
./ragit/vendor/google-oauth/Cargo.toml
./ragit/vendor/google-oauth/examples/async_client/Cargo.toml
./ragit/vendor/google-oauth/examples/blocking/Cargo.toml
./ragit/vendor/google-ai-rs/Cargo.toml
./ragit/vendor/google-ai-rs/google-ai-rs/Cargo.toml
./ragit/vendor/google-ai-rs/google-ai-schema-derive/Cargo.toml
./zed/crates/google_ai/Cargo.toml
./ragit/vendor/meta-introspector/solfunmeme-dioxus/crates/gemini_cli_lib/Cargo.toml

see also
./pica/api/src/logic/connection_oauth_definition.rs
./pica/api/src/logic/oauth.rs
./pica/osentities/src/algebra/oauth.rs
./pica/osentities/src/domain/connection/connection_oauth_definition.rs
./pica/osentities/src/domain/secret/oauth_secret.rs
./goose/crates/goose-mcp/src/google_drive/oauth_pkce.rs
./goose/crates/goose/examples/databricks_oauth.rs
./goose/crates/goose/src/providers/oauth.rs
./goose/crates/mcp-client/src/oauth.rs
./goose/crates/mcp-client/src/oauth_tests.rs


./libminizinc/vendor/oauth2/examples/github.rs
./solfunmeme-dioxus/vendor/axum/examples/oauth/src/main.rs

./mcp/monomcp/turbomcp/crates/turbomcp/examples/09_oauth_authentication.rs
./mcp/monomcp/turbomcp/crates/turbomcp/tests/oauth_redirect_validation_tests.rs
./mcp/read_oauth_credentials/src/main.rs
./aichat/src/utils/oauth.rs

./rig/rig-core/examples/gemini_agent.rs
./rig/rig-core/examples/gemini_embeddings.rs
./rig/rig-core/examples/gemini_extractor.rs
./rig/rig-core/examples/gemini_streaming.rs
./rig/rig-core/examples/gemini_streaming_with_tools.rs

./rig/rig-core/src/providers/gemini/client.rs
./rig/rig-core/src/providers/gemini/completion.rs
./rig/rig-core/src/providers/gemini/embedding.rs
./rig/rig-core/src/providers/gemini/mod.rs
./rig/rig-core/src/providers/gemini/streaming.rs
./rig/rig-core/src/providers/gemini/transcription.rs

./agentgateway/crates/agentgateway/src/llm/gemini.rs
./goose/crates/goose/src/providers/gemini_cli.rs
./anda/anda_engine/src/model/gemini.rs
./ragit/vendor/meta-introspector/solfunmeme-dioxus/crates/gemini_cli_lib/src/gemini/artifact.rs
./ragit/vendor/meta-introspector/solfunmeme-dioxus/crates/gemini_cli_lib/src/gemini/mod.rs
./ragit/vendor/meta-introspector/solfunmeme-dioxus/crates/gemini_cli_lib/src/lib.rs

./ragit/vendor/amazon-q-developer-cli/crates/chat-cli/src/api_client/gemini_client.rs
./ragit/vendor/amazon-q-developer-cli/crates/gemini-auth/src/main.rs
./ragit/vendor/llmclient/src/gemini.rs
./ragit/vendor/ask_gemini/src/lib.rs
./ragit/vendor/ask_gemini/src/structs.rs
./async-llm/examples/gemini.rs
./solfunmeme-dioxus/crates/gemini_cli_lib/src/gemini/artifact.rs
./solfunmeme-dioxus/crates/gemini_cli_lib/src/gemini/mod.rs
./solfunmeme-dioxus/crates/gemini_cli_lib/src/lib.rs
./libminizinc/crates/gemini_utils/src/lib.rs
./libminizinc/crates/gemini_utils/src/macro_parser/comma_separated_exprs.rs
./libminizinc/crates/gemini_utils/src/macro_parser/gemini_eprintln_input.rs
./libminizinc/crates/gemini_utils/src/macro_parser/mod.rs
./libminizinc/crates/gemini_utils/src/macro_parser/named_args_input.rs
./libminizinc/crates/gemini_utils/src/string_processor/char_handlers/handle_backslash.rs
./libminizinc/crates/gemini_utils/src/string_processor/char_handlers/handle_curly_brace.rs
./libminizinc/crates/gemini_utils/src/string_processor/char_handlers/handle_other_char.rs
./libminizinc/crates/gemini_utils/src/string_processor/char_handlers/mod.rs
./libminizinc/crates/gemini_utils/src/string_processor/mod.rs
./libminizinc/crates/gemini_utils/src/string_processor/processing_context.rs
./libminizinc/crates/gemini_utils/src/string_processor/segment_appender.rs
./libminizinc/crates/gemini_utils/src/token_generator/generate_eprintln_tokens.rs
./libminizinc/crates/gemini_utils/src/token_generator/mod.rs
./libminizinc/crates/gemini_utils/src/argument_mapper.rs
./libminizinc/crates/gemini_utils/build.rs
./libminizinc/crates/gemini_utils_test/src/lib.rs
./libminizinc/crates/gemini_utils_test/src/tests/basic_logging.rs
./libminizinc/crates/gemini_utils_test/src/tests/emoji_handling.rs
./libminizinc/crates/gemini_utils_test/src/tests/variable_handling.rs
./libminizinc/crates/gemini_utils_test/src/tests/kantspel.rs
./libminizinc/crates/gemini_utils_test/src/tests/mod.rs
./libminizinc/crates/mini-act/src/gemini_context_args.rs
./libminizinc/crates/launchpad/src/dum_wrappers/gemini_cli_runner.rs
./libminizinc/crates/launchpad/src/gemini_cli_options.rs
./libminizinc/crates/launchpad/src/orchestrator/gemini_cli_manager.rs
./libminizinc/crates/tmux_controller/src/gemini_commands.rs
./libminizinc/crates/gemini_cli_manager/src/lib.rs
./libminizinc/crates/solfunmeme-core/src/dum_wrappers/gemini_cli_runner.rs
./libminizinc/crates/solfunmeme-core/src/gemini_cli_options.rs
./libminizinc/crates/solfunmeme-core/src/gemini_commands.rs
./mcp/monomcp/siumai/src/params/gemini.rs
./mcp/monomcp/siumai/src/providers/gemini/chat.rs
./mcp/monomcp/siumai/src/providers/gemini/client.rs
./mcp/monomcp/siumai/src/providers/gemini/code_execution.rs
./mcp/monomcp/siumai/src/providers/gemini/embeddings.rs
./mcp/monomcp/siumai/src/providers/gemini/files.rs
./mcp/monomcp/siumai/src/providers/gemini/mod.rs
./mcp/monomcp/siumai/src/providers/gemini/model_constants.rs
./mcp/monomcp/siumai/src/providers/gemini/models.rs
./mcp/monomcp/siumai/src/providers/gemini/request.rs
./mcp/monomcp/siumai/src/providers/gemini/streaming.rs
./mcp/monomcp/siumai/src/providers/gemini/types.rs

./aichat/src/client/gemini.rs

review all these files

1. what do they do
2. what do they overlap with
3. how can we integrate them into aichat?
4. how can we abstract them?

make a plan. follow itil, iso9k, gmp, c4, uml, use mermaid and plantuml. make a design. 
