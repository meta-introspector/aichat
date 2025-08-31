# Plan for Integrating gemini-cli into aichat

This document outlines the staged approach for integrating `gemini-cli` functionalities, particularly OAuth-based authentication, into `aichat`. The plan adheres to principles from ITIL, ISO9K, GMP, and utilizes concepts from C4 model and UML for design descriptions.

## Overall Approach (ITIL, ISO9K, GMP Principles)

*   **Service Strategy/Design (ITIL):** Define the scope, objectives, and high-level design for Gemini integration within `aichat`. This involves moving from API key-based authentication to OAuth, and potentially leveraging other `gemini-cli` features.
*   **Quality Management (ISO9K, GMP):** Ensure the integration is robust, maintainable, and adheres to coding standards. This will involve careful analysis, modular design, and verification steps including unit tests, integration tests, and adherence to project-specific linting and build processes.
*   **Structured Design (C4, UML):** The design will be described at different levels of abstraction (Context, Container, Component, Code) using textual descriptions that can be easily translated into Mermaid or PlantUML diagrams.

## Detailed Review of External Codebases and Reusability

During Stage 2, a comprehensive review of various Rust and TypeScript codebases related to Gemini and OAuth was conducted. The goal was to identify existing capabilities, overlaps, and potential areas for reuse or inspiration for `aichat`.

### OAuth Implementations

*   **`aichat`'s current OAuth (`src/auth/oauth.rs`, `src/auth/credential_store.rs`):** This implementation is robust, uses PKCE, and is compatible with `gemini-cli`'s credential storage location (`~/.gemini/oauth_creds.json`). Dynamic port selection has been integrated.
*   **`goose/crates/goose/src/providers/oauth.rs`:** This is a highly robust and generic OAuth client. It features comprehensive token caching and refresh logic, uses `axum` for its temporary web server, and handles dynamic port selection. **Recommendation:** This crate offers a more complete and battle-tested solution. It is strongly recommended to either adopt this crate directly or heavily borrow its patterns and components to enhance `aichat`'s custom OAuth implementation. This would significantly improve robustness, maintainability, and feature completeness (e.g., better token expiration handling, more flexible redirect server).
*   **`r-google-oauth2` and `rs-gapi-oauth`:** These projects provide valuable examples of OAuth flows and utility functions. `r-google-oauth2/src/util.rs` provided inspiration for dynamic port selection. `rs-gapi-oauth` demonstrates structured management of different authentication types (user, service account) and robust testing practices for OAuth flows. These serve as good references for structuring `aichat`'s authentication module and testing.
*   **`gemini-cli/packages/core/src/code_assist/oauth2.ts` (TypeScript):** This was the initial reference for `gemini-cli`'s OAuth. Its core logic is similar to `aichat`'s, but `aichat`'s Rust implementation is now more advanced (PKCE, dynamic port selection).

### Gemini Client Implementations

*   **`aichat`'s current Gemini client (`src/client/gemini.rs`):** This client is API key-based and interacts with the public Gemini API (`generativelanguage.googleapis.com`).
*   **`gemini-cli`'s `CodeAssistServer` (TypeScript):** This client uses a specialized internal Google API endpoint (`cloudcode-pa.googleapis.com`) and is tightly coupled with Google's Code Assist platform. **Conclusion:** Due to its specific target API and platform, direct porting of `CodeAssistServer` is not feasible or desirable if `aichat` is to continue using the public Gemini API.
*   **`rig/rig-core/src/providers/gemini/client.rs` and `ragit/vendor/amazon-q-developer-cli/crates/chat-cli/src/api_client/gemini_client.rs`:** These are other API key-based clients for the public Gemini API, offering similar functionality to `aichat`'s existing client.
*   **`mcp/monomcp/siumai/src/providers/gemini/client.rs`:** This client is highly modular and feature-rich, supporting capabilities like file management, model listing, and advanced generation configurations. **Recommendation:** This is an excellent source of inspiration for future enhancements to `aichat`'s Gemini client, particularly if `aichat` aims to support more advanced Gemini features (e.g., multimodal input, tool use, file API interactions).
*   **`google-ai-rs` (gRPC-based):** A type-safe gRPC client for Google AI APIs. **Recommendation:** This could be a valuable reference if `aichat` ever considers moving to gRPC for performance or specific API interactions, or if service account authentication is required.
*   **`ask_gemini`:** A straightforward `reqwest`-based Gemini client. It serves as a good comparison point and a source of inspiration for minor improvements in `aichat`'s client, especially regarding error handling with `thiserror`.
*   **`async-llm/examples/gemini.rs`:** Provides comprehensive examples demonstrating various Gemini API features, including structured outputs, tool calls, and multimodal input. **Recommendation:** This is an invaluable guide for extending `aichat`'s Gemini capabilities and implementing advanced features.

### CLI Options and Utilities

*   **`mini-act/src/gemini_context_args.rs`, `launchpad/src/gemini_cli_options.rs`, `solfunmeme-dioxus/crates/gemini_cli_lib/src/gemini/artifact.rs`, etc.:** These files define comprehensive CLI options for Gemini tools, often using `clap`. **Recommendation:** These serve as excellent inspiration for structuring `aichat`'s CLI arguments and identifying potential future features such as sandboxing, telemetry, and proxy support. The `proxy` option is particularly relevant for `aichat`'s network requests.
*   **`gemini_utils`:** A procedural macro crate for Gemini-related utilities. **Recommendation:** Its source code should be reviewed for specific text processing or code generation utilities that might be useful for `aichat`.
*   **`gemini_cli_manager` and `tmux_controller`:** These crates are designed for managing Gemini CLIs within `tmux` sessions. **Conclusion:** While not directly relevant for `aichat`'s core LLM interaction, they offer insights into advanced CLI orchestration and process management patterns.

## Staged Plan

### Stage 1: Initial Assessment & Information Gathering (Completed)

*   **Objective:** Gain a high-level understanding of the existing `aichat` Gemini integration, the `gemini-cli` structure, and the various OAuth and Gemini-related files provided in `todo.md`.
*   **Actions Taken:**
    1.  Reviewed `aichat/src/client/gemini.rs` to understand its current capabilities (API key-based chat completions and embeddings).
    2.  Reviewed `aichat/Cargo.toml` to understand its dependencies.
    3.  Attempted to review `gemini-cli/Cargo.toml` and discovered `gemini-cli` is likely a workspace with crates under `gemini-cli/crates/`.
    4.  Listed contents of `gemini-cli/crates/` to confirm `solfunmeme-banner` crate.
*   **Expected Output:** A summary of the current state, identified key areas for integration, and a refined plan for Stage 2.

### Stage 2: Detailed Analysis & Overlap Identification (Completed)

*   **Objective:** Deep dive into the content of the identified files to understand their specific functionalities, identify overlaps, and pinpoint potential areas for abstraction and integration. This stage also involved understanding the TypeScript/JavaScript implementation of `gemini-cli` for comparison.
*   **Actions:**
    1.  **OAuth Analysis (TypeScript/JavaScript & Rust):**
        *   Read `gemini-cli/packages/core/src/code_assist/oauth2.ts` to understand the full OAuth flow implemented in `gemini-cli` (authorization, token exchange, refresh, storage).
        *   Read files related to `~/.gemini/oauth_creds.json` and `google-oauth` in Rust (`aichat/src/auth/oauth.rs`, `aichat/src/auth/credential_store.rs`, `ragit/vendor/google-oauth/Cargo.toml`, `mcp/read_oauth_credentials/src/main.rs`) to identify existing Rust capabilities.
        *   Understood the OAuth flow, credential storage, and token management in both languages.
    2.  **Gemini Client Analysis (TypeScript/JavaScript & Rust):**
        *   Read relevant TypeScript/JavaScript files in `gemini-cli` that handle Gemini API calls (`gemini-cli/packages/core/src/core/geminiChat.ts`, `gemini-cli/packages/core/src/core/contentGenerator.ts`, `gemini-cli/packages/core/src/code_assist/codeAssist.ts`, `gemini-cli/packages/core/src/code_assist/server.ts`).
        *   Read files related to Gemini API clients in Rust (`rig/rig-core/src/providers/gemini/client.rs`, `ragit/vendor/amazon-q-developer-cli/crates/chat-cli/src/api_client/gemini_client.rs`, `mcp/monomcp/siumai/src/providers/gemini/client.rs`, and `aichat/src/client/gemini.rs`).
        *   Compared their functionalities and identified features to port.
    3.  **Utility/Common Libraries Analysis (Rust):**
        *   Reviewed `gemini_utils` and `gemini_cli_lib` crates for reusable components.
    4.  **Overlap Identification:** Documented redundant functionalities, similar API calls, and potential shared data structures across the analyzed files in both languages.
*   **Expected Output:**
    *   `aichat` already has a robust Rust OAuth implementation (`aichat/src/auth/oauth.rs` and `aichat/src/auth/credential_store.rs`) that is compatible with `gemini-cli`'s credential storage (`~/.gemini/oauth_creds.json`).
    *   `gemini-cli`'s `CodeAssistServer` uses a specialized internal Gemini API endpoint (`cloudcode-pa.googleapis.com`) for its OAuth-based interactions, which is different from the public Gemini API (`generativelanguage.googleapis.com`) used by `aichat` and other Rust clients reviewed. Therefore, direct porting of `CodeAssistServer` is not feasible or desirable if `aichat` is to continue using the public API.
    *   The `siumai` Gemini client (`mcp/monomcp/siumai/src/providers/gemini/client.rs`) is highly modular and feature-rich, offering capabilities like file management and advanced generation configurations, which could be considered for future enhancements to `aichat`'s Gemini client.

### Stage 3: Design & Abstraction (C4 Model & UML-like Descriptions)

*   **Objective:** Propose a modular and extensible design for integrating OAuth-based authentication into `aichat`'s existing Gemini client, focusing on clear interfaces and separation of concerns. This will enable `aichat` to use OAuth with the public Gemini API. We will also consider how to integrate advanced features inspired by other reviewed projects.
*   **Actions:**
    1.  **Context View (C4):**
        *   **System:** `aichat`
        *   **Users:** CLI users interacting with `aichat`.
        *   **External Systems:** Google Gemini API, Google OAuth 2.0.
        *   **Description:** `aichat` will interact with the Google Gemini API for AI functionalities, leveraging Google OAuth 2.0 for secure authentication. Future enhancements may involve integrating with other LLM providers or advanced AI capabilities.
        *   **Mermaid/PlantUML Sketch:**
            ```mermaid
            graph TD
                User --> Aichat
                Aichat --> GoogleGeminiAPI
                Aichat --> GoogleOAuth2
            ```
    2.  **Container View (C4):**
        *   **Containers:** `aichat` CLI application, OAuth Credential Store (e.g., `~/.gemini/oauth_creds.json`).
        *   **Description:** The `aichat` CLI application will manage Gemini API interactions and OAuth authentication internally. OAuth credentials will be stored externally. Future container considerations may include integration with external LLM management libraries (like `llm`) or local LLM inference engines (like `candle`).
        *   **Mermaid/PlantUML Sketch:**
            ```mermaid
            graph TD
                AichatCLI --> GeminiClientModule
                GeminiClientModule --> GoogleGeminiAPI
                GeminiClientModule --> OAuthCredentialStore
                OAuthCredentialStore --> GoogleOAuth2
            ```
    3.  **Component View (C4):**
        *   **Components within `GeminiClientModule`:**
            *   `OAuthAuthenticator`: Manages the OAuth 2.0 authorization code flow, including generating auth URLs, handling redirects, exchanging codes for tokens, and refreshing tokens. It will utilize the `oauth2` Rust crate, with inspiration from `goose/crates/goose/src/providers/oauth.rs` for robustness and dynamic port handling.
            *   `CredentialStore`: Responsible for securely reading from and writing to `~/.gemini/oauth_creds.json`.
            *   `GeminiApiClient`: The main client for interacting with the Gemini API. It will be refactored to accept an `Authenticator` trait for obtaining access tokens. Future enhancements may involve adopting a more modular design inspired by `mcp/monomcp/siumai` or integrating a unified LLM library like `llm`.
            *   `ModelConfigLoader`: (Existing in `aichat`, will remain) Loads Gemini model configurations.
        *   **Description:** The `GeminiClientModule` will encapsulate the `OAuthAuthenticator` for authentication and `CredentialStore` for token persistence. The `GeminiApiClient` will be refactored to use a generic `Authenticator` trait, allowing it to support both API key and OAuth authentication. Advanced features from other reviewed projects will be considered for future integration.
        *   **Mermaid/PlantUML Sketch:**
            ```mermaid
            graph TD
                subgraph GeminiClientModule
                    OAuthAuthenticator --> CredentialStore
                    GeminiApiClient --> OAuthAuthenticator
                    GeminiApiClient --> ModelConfigLoader
                end
                AichatCLI --> GeminiClientModule
            ```
    4.  **Code View (UML-like):**
        *   **Key Structs/Traits:**
            *   `trait Authenticator`: Defines the interface for obtaining an access token (e.g., `authenticate() -> Result<AccessToken>`).
            *   `struct OAuthAuthenticator`: Implements `Authenticator`. Encapsulates the OAuth 2.0 flow using the `oauth2` crate and interacts with `CredentialStore`. It will incorporate dynamic port selection and user info fetching.
            *   `struct ApiKeyAuthenticator`: Implements `Authenticator`. (Existing functionality).
            *   `struct GeminiClient`: (Refactored) Takes a `Box<dyn Authenticator>` as a dependency.
            *   `trait ChatCapability`: Defines interface for chat completions.
            *   `trait EmbeddingCapability`: Defines interface for embeddings.
            *   `trait FileManagementCapability`: Defines interface for file management (inspired by `mcp/siumai` - *future consideration*).
            *   `struct GeminiChatClient`: Implements `ChatCapability`.
            *   `struct GeminiEmbeddingClient`: Implements `EmbeddingCapability`.
            *   `struct GeminiFileManager`: Implements `FileManagementCapability` (*future consideration*).
        *   **Description:** Introduce a generic `Authenticator` trait for flexible authentication. The `GeminiClient` will be refactored to use this trait. Specialized capability clients (like `GeminiChatClient`, `GeminiEmbeddingClient`) can be composed within `GeminiClient` for modularity and extensibility. File management capabilities, inspired by `siumai`, and advanced features like structured outputs and tool calls (inspired by `async-llm`) can be considered for future enhancements.
        *   **Mermaid/PlantUML Sketch (Class Diagram):**
            ```mermaid
            classDiagram
                class Authenticator {
                    <<trait>>
                    +authenticate(): Result<AccessToken>
                }
                class OAuthAuthenticator {
                    +new(config: OAuthConfig, credential_store: Arc<CredentialStore>)
                    +authenticate(): Result<AccessToken>
                }
                class ApiKeyAuthenticator {
                    +new(api_key: String)
                    +authenticate(): Result<AccessToken>
                }
                class CredentialStore {
                    +read_credentials(): Result<Credentials>
                    +write_credentials(creds: Credentials): Result<()>
                }
                class GeminiClient {
                    -authenticator: Box<dyn Authenticator>
                    -chat_client: GeminiChatClient
                    -embedding_client: GeminiEmbeddingClient
                    -file_manager: GeminiFileManager
                    +new(authenticator: Box<dyn Authenticator>, ...)
                }
                class GeminiChatClient {
                    +chat_completions(...)
                }
                class GeminiEmbeddingClient {
                    +embeddings(...)
                }
                class GeminiFileManager {
                    +upload_file(...)
                    +list_files(...)
                }

                Authenticator <|-- OAuthAuthenticator
                Authenticator <|-- ApiKeyAuthenticator
                GeminiClient --> Authenticator
                GeminiClient --> GeminiChatClient
                GeminiClient --> GeminiEmbeddingClient
                GeminiClient --> GeminiFileManager
                OAuthAuthenticator --> CredentialStore

                ChatCapability <|.. GeminiChatClient
                EmbeddingCapability <|.. GeminiEmbeddingClient
                FileManagementCapability <|.. GeminiFileManager
            ```
*   **Expected Output:** A comprehensive design document with textual descriptions for C4 views and UML-like component interactions.

### Stage 4: Implementation Plan & Verification

*   **Objective:** Outline the steps for implementing the design and verifying the changes, and demonstrate sending a poem request to Gemini using the OAuth-enabled client.
*   **Actions:**
    1.  **Refine `aichat`'s OAuth Logic:**
        *   Implement dynamic port selection for the redirect server in `aichat/src/auth/oauth.rs` (already completed, inspired by `gemini-cli`'s TypeScript implementation and `goose/crates/goose/src/providers/oauth.rs`).
        *   Implement user info fetching and caching in `aichat/src/auth/oauth.rs` for improved user experience (already completed, inspired by `gemini-cli`'s TypeScript implementation and `r-google-oauth2`).
    2.  **Refactor `aichat/src/client/gemini.rs`:**
        *   Introduce an `Authenticator` trait (if not already present) that `ApiKeyAuthenticator` and `OAuthAuthenticator` can implement.
        *   Modify `GeminiClient` to accept a `Box<dyn Authenticator>` as a dependency, allowing it to use either API key or OAuth for authentication.
    3.  **Integrate CLI Commands:**
        *   Add new CLI commands to `aichat` for initiating the OAuth login flow (e.g., `aichat auth login`).
        *   Add commands for managing OAuth credentials (e.g., `aichat auth logout`).
    4.  **Implement Gemini Request with OAuth:**
        *   Modify `aichat`'s main logic to use the OAuth-enabled Gemini client when configured.
        *   Demonstrate sending a simple text request (e.g., a poem request) to Gemini using the `aichat` CLI.
    5.  **Testing:**
        *   Write unit tests for the refined `OAuthAuthenticator` and the modified `GeminiClient`.
        *   Develop integration tests for the end-to-end OAuth flow and Gemini API calls using OAuth.
    6.  **Verification:**
        *   Run `cargo check`, `cargo build`, and `cargo test`.
        *   Execute project-specific linting (if any).
*   **Expected Output:** A step-by-step implementation roadmap, including specific file modifications and testing strategies, and a successful demonstration of Gemini interaction via OAuth.

### Stage 5: Quality Assurance Plan

*   **Objective:** Ensure the robustness, reliability, and user-friendliness of the integrated OAuth and Gemini functionalities.
*   **Actions:**
    1.  **Unit Testing:**
        *   Verify individual components of the OAuth flow (`OAuthAuthenticator`, `CredentialStore`) and the `GeminiClient` (with both API key and OAuth authentication).
        *   Focus on edge cases, error handling, and token management.
    2.  **Integration Testing:**
        *   Test the end-to-end OAuth flow, from initiating login to successful token acquisition and storage.
        *   Verify successful Gemini API calls using OAuth-obtained tokens for various functionalities (chat, embeddings).
        *   Test scenarios like token expiration and refresh.
    3.  **User Acceptance Testing (UAT) / "Noob Guide" Verification:**
        *   Follow the "Noob Guide for Interactive Credential Import" (to be created) to ensure the process is intuitive and works as expected for new users.
        *   Gather feedback from non-technical users on the ease of use and clarity of instructions.
    4.  **Performance Testing:**
        *   (Future Consideration) Measure the latency and throughput of Gemini API calls with OAuth authentication.
    5.  **Security Review:**
        *   (Future Consideration) Review the OAuth implementation for common vulnerabilities (e.g., redirect URI validation, state parameter usage).
    6.  **Regression Testing:**
        *   After any new feature or bug fix, re-run all existing tests to ensure no existing functionality is broken.
*   **Expected Output:** A thoroughly tested and verified integration, with clear documentation for users on how to set up credentials.

### Noob Guide: Interactive Gemini Credential Setup

This guide will walk you through the process of setting up your Gemini credentials for `aichat` interactively. This is designed for users who are new to command-line tools and OAuth authentication.

#### Option 1: Using OAuth (Recommended for most users)

OAuth provides a secure and convenient way to authenticate with Gemini without directly handling API keys.

1.  **Initiate the Login Flow:**
    Open your terminal and run the following command:
    ```bash
    aichat auth login
    ```
    *What happens:* This command will open your web browser to a Google authentication page.
2.  **Approve Access:**
    In your web browser, you will be prompted to sign in with your Google account and grant `aichat` permission to access the Gemini API.
    *   Select the Google account you wish to use.
    *   Review the permissions requested by `aichat` and click "Allow" or "Accept".
3.  **Return to Terminal:**
    After approving access, your browser will redirect to a local server started by `aichat`. You should see a success message in your browser and in your terminal.
    *What happens:* `aichat` will automatically capture the necessary tokens and securely store them on your system (usually in `~/.gemini/oauth_creds.json`).
4.  **Verify Setup:**
    You can now try to use `aichat` with Gemini. For example:
    ```bash
    aichat chat "Tell me a short poem about a cat."
    ```
    If everything is set up correctly, `aichat` will respond with a poem.

#### Option 2: Using an API Key (Advanced Users / Specific Use Cases)

If you prefer to use an API key directly, follow these steps. This method is generally less secure than OAuth for interactive use.

1.  **Obtain Your API Key:**
    *   Go to the Google AI Studio website: [https://aistudio.google.com/](https://aistudio.google.com/)
    *   Log in with your Google account.
    *   Navigate to the "Get API Key" section.
    *   Create a new API key or copy an existing one.
    *   **Important:** Treat your API key like a password. Do not share it or expose it in public repositories.
2.  **Configure `aichat` with the API Key:**
    You can set your API key as an environment variable. This is the recommended way to use API keys with `aichat`.
    ```bash
    export GEMINI_API_KEY="YOUR_API_KEY_HERE"
    ```
    *Replace `YOUR_API_KEY_HERE` with the actual API key you obtained.*
    *Note:* This environment variable will only be set for your current terminal session. To make it permanent, you'll need to add it to your shell's configuration file (e.g., `.bashrc`, `.zshrc`, `.profile`).
3.  **Verify Setup:**
    You can now try to use `aichat` with Gemini. For example:
    ```bash
    aichat chat "Tell me a short poem about a dog."
    ```
    If everything is set up correctly, `aichat` will respond with a poem.

---
**Next Step:** Proceed with Stage 4: Implementation Plan & Verification, starting with refactoring `aichat/src/client/gemini.rs` to use the `Authenticator` trait.

