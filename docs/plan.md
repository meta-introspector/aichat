# Plan for Integrating gemini-cli into aichat

This document outlines the staged approach for integrating `gemini-cli` functionalities, particularly OAuth-based authentication, into `aichat`. The plan adheres to principles from ITIL, ISO9K, GMP, and utilizes concepts from C4 model and UML for design descriptions.

## Overall Approach (ITIL, ISO9K, GMP Principles)

*   **Service Strategy/Design (ITIL):** Define the scope, objectives, and high-level design for Gemini integration within `aichat`. This involves moving from API key-based authentication to OAuth, and potentially leveraging other `gemini-cli` features.
*   **Quality Management (ISO9K, GMP):** Ensure the integration is robust, maintainable, and adheres to coding standards. This will involve careful analysis, modular design, and verification steps including unit tests, integration tests, and adherence to project-specific linting and build processes.
*   **Structured Design (C4, UML):** The design will be described at different levels of abstraction (Context, Container, Component, Code) using textual descriptions that can be easily translated into Mermaid or PlantUML diagrams.

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

*   **Objective:** Propose a modular and extensible design for integrating OAuth-based authentication into `aichat`'s existing Gemini client, focusing on clear interfaces and separation of concerns. This will enable `aichat` to use OAuth with the public Gemini API.
*   **Actions:**
    1.  **Context View (C4):**
        *   **System:** `aichat`
        *   **Users:** CLI users interacting with `aichat`.
        *   **External Systems:** Google Gemini API, Google OAuth 2.0.
        *   **Description:** `aichat` will interact with the Google Gemini API for AI functionalities, leveraging Google OAuth 2.0 for secure authentication.
        *   **Mermaid/PlantUML Sketch:**
            ```mermaid
            graph TD
                User --> Aichat
                Aichat --> GoogleGeminiAPI
                Aichat --> GoogleOAuth2
            ```
    2.  **Container View (C4):**
        *   **Containers:** `aichat` CLI application, OAuth Credential Store (e.g., `~/.gemini/oauth_creds.json`).
        *   **Description:** The `aichat` CLI application will manage Gemini API interactions and OAuth authentication internally. OAuth credentials will be stored externally.
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
            *   `OAuthAuthenticator`: Manages the OAuth 2.0 authorization code flow, including generating auth URLs, handling redirects, exchanging codes for tokens, and refreshing tokens. It will utilize the `oauth2` Rust crate.
            *   `CredentialStore`: Responsible for securely reading from and writing to `~/.gemini/oauth_creds.json`.
            *   `GeminiApiClient`: The main client for interacting with the Gemini API. It will be refactored to accept an `Authenticator` trait for obtaining access tokens.
            *   `ModelConfigLoader`: (Existing in `aichat`, will remain) Loads Gemini model configurations.
        *   **Description:** The `GeminiClientModule` will encapsulate the `OAuthAuthenticator` for authentication and `CredentialStore` for token persistence. The `GeminiApiClient` will be refactored to use a generic `Authenticator` trait, allowing it to support both API key and OAuth authentication.
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
            *   `struct OAuthAuthenticator`: Implements `Authenticator`. Encapsulates the OAuth 2.0 flow using the `oauth2` crate and interacts with `CredentialStore`.
            *   `struct ApiKeyAuthenticator`: Implements `Authenticator`. (Existing functionality).
            *   `struct GeminiClient`: (Refactored) Takes a `Box<dyn Authenticator>` as a dependency.
            *   `trait ChatCapability`: Defines interface for chat completions.
            *   `trait EmbeddingCapability`: Defines interface for embeddings.
            *   `trait FileManagementCapability`: Defines interface for file management (inspired by `mcp/siumai` - *future consideration*).
            *   `struct GeminiChatClient`: Implements `ChatCapability`.
            *   `struct GeminiEmbeddingClient`: Implements `EmbeddingCapability`.
            *   `struct GeminiFileManager`: Implements `FileManagementCapability` (*future consideration*).
        *   **Description:** Introduce a generic `Authenticator` trait for flexible authentication. The `GeminiClient` will be refactored to use this trait. Specialized capability clients (like `GeminiChatClient`, `GeminiEmbeddingClient`) can be composed within `GeminiClient` for modularity and extensibility. File management capabilities, inspired by `siumai`, can be considered for future enhancements.
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

*   **Objective:** Outline the steps for implementing the design and verifying the changes.
*   **Actions:**
    1.  **Refine `aichat`'s OAuth Logic:**
        *   Implement dynamic port selection for the redirect server in `aichat/src/auth/oauth.rs` (similar to `gemini-cli`'s TypeScript implementation).
        *   Consider adding user info fetching and caching in `aichat/src/auth/oauth.rs` for improved user experience.
    2.  **Refactor `aichat/src/client/gemini.rs`:**
        *   Introduce an `Authenticator` trait (if not already present) that `ApiKeyAuthenticator` and `OAuthAuthenticator` can implement.
        *   Modify `GeminiClient` to accept a `Box<dyn Authenticator>` as a dependency, allowing it to use either API key or OAuth for authentication.
    3.  **Integrate CLI Commands:**
        *   Add new CLI commands to `aichat` for initiating the OAuth login flow (e.g., `aichat auth login`).
        *   Add commands for managing OAuth credentials (e.g., `aichat auth logout`).
    4.  **Testing:**
        *   Write unit tests for the refined `OAuthAuthenticator` and the modified `GeminiClient`.
        *   Develop integration tests for the end-to-end OAuth flow and Gemini API calls using OAuth.
    5.  **Verification:**
        *   Run `cargo check`, `cargo build`, and `cargo test`.
        *   Execute project-specific linting (if any).
*   **Expected Output:** A step-by-step implementation roadmap, including specific file modifications and testing strategies.

---
**Next Step:** Proceed with Stage 3: Design & Abstraction, focusing on the refined plan.

