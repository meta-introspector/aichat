# SOP: Implementing the Google Cloud OAuth Scope Management Meta Client (CRQ 2)

This Standard Operating Procedure outlines the steps for implementing the "Meta Client" as described in CRQ 2. This client will enable programmatic management of Google Cloud OAuth client configurations, specifically focusing on "Authorized redirect URIs."

## Phase 1: Setup and Core OAuth Flow

### 1.1 Define CLI Command
- **Action:** Add a new subcommand to `src/cli.rs` to serve as the entry point for the meta client functionality.
- **Example:** `aichat auth manage-google-oauth`
- **Details:** This command will handle parsing arguments related to project ID, client ID, and redirect URI operations.

### 1.2 Meta Client OAuth Configuration
- **Action:** Create a dedicated configuration structure for the meta client's OAuth credentials.
- **Location:** Consider a new module like `src/config/oauth_meta.rs` or extend `src/config/mod.rs`.
- **Details:** This structure will hold the `client_id` and `client_secret` for the meta client itself. These credentials must be obtained from Google Cloud Platform by registering a separate OAuth client for this management purpose.
- **Action:** Implement a mechanism to load these credentials securely.
- **Details:** This will be similar to `load_gemini_oauth_config` but specifically for the meta client's credentials, likely reading from a dedicated JSON file (e.g., `clients/meta-client/client_secret.json`).

### 1.3 Implement Meta Client OAuth Flow
- **Action:** Adapt and leverage existing OAuth flow components within `src/auth/oauth_split/`.
- **Relevant Files:** `oauth_client_setup.rs`, `web_auth_flow.rs`, `web_flow_token_exchange.rs`.
- **Details:**
    - The OAuth flow for the meta client will use its specific `client_id`, `client_secret`, and the `https://www.googleapis.com/auth/cloud-platform` scope. This broad scope is necessary for managing project resources.
    - Ensure the meta client's access and refresh tokens are saved securely, ideally in a separate file from the primary application's credentials (e.g., `~/.zos/meta_oauth_creds.json`) using the `CredentialStore` mechanism.

## Phase 2: Google Cloud API Integration

### 2.1 Select Google Cloud Rust SDK
- **Action:** Evaluate and select an appropriate Google Cloud Rust SDK from the options listed in CRQ 2.
- **Considerations:**
    - `https://github.com/googleapis/google-cloud-rust/tree/main`
    - `https://github.com/0Itsuki0/google-api-rust-client`
    - `https://github.com/abdolence/gcloud-sdk-rs`
- **Details:** The chosen SDK must provide robust functionality for interacting with the Google Cloud APIs related to OAuth client management. This may require a deeper dive into each SDK's documentation and capabilities.

### 2.2 Implement OAuth Client Management Logic
- **Action:** Develop functions to interact with the Google Cloud APIs via the selected SDK.
- **Core Functionality:**
    - List existing OAuth 2.0 client IDs within a specified Google Cloud Project.
    - Retrieve detailed configurations for a given OAuth 2.0 client ID.
    - Add new "Authorized redirect URIs" to an existing OAuth 2.0 client ID.
    - Remove "Authorized redirect URIs" from an existing OAuth 2.0 client ID.
- **Optional Functionality:**
    - Manage "Authorized JavaScript origins."
    - Manage other aspects of OAuth 2.0 client configurations as needed.
- **Authentication:** These functions will use the access token obtained by the meta client's OAuth flow.

## Phase 3: User Interface and Testing

### 3.1 CLI User Interface
- **Action:** Design and implement the command-line interface for the `aichat auth manage-google-oauth` command.
- **Details:** Provide clear and intuitive options for users to specify:
    - Google Cloud Project ID.
    - Target OAuth client ID.
    - Redirect URIs to add or remove.
    - Confirmation prompts for destructive actions.

### 3.2 Error Handling and Feedback
- **Action:** Implement comprehensive error handling for all stages of the meta client's operation.
- **Details:** Provide informative error messages to the user, guiding them on how to resolve issues (e.g., invalid credentials, API errors, network issues).

### 3.3 Testing
- **Action:** Write thorough unit and integration tests for the entire meta client functionality.
- **Scope:** Cover the OAuth flow, secure credential storage, and interactions with the Google Cloud APIs.
- **Tools:** Utilize existing testing frameworks and methodologies within the project.

## Integration with Gemini Client

Once the "Meta Client" is successfully implemented, the process for setting up and managing a new Gemini client will be streamlined:

1.  **Create Gemini OAuth Client (via Meta Client):** Users will leverage the `aichat auth manage-google-oauth` command to programmatically create a new OAuth 2.0 client in Google Cloud Platform specifically for the Gemini client. This will involve specifying the necessary redirect URIs and obtaining the `client_id` and `client_secret` for the Gemini client.
2.  **Configure Gemini Client:** The obtained `client_id` and `client_secret` for the Gemini client will then be used to configure the application's Gemini client (e.g., by updating a configuration file or providing them as environment variables).
3.  **Authenticate Gemini Client:** The Gemini client will then execute its own OAuth flow (as currently implemented) to obtain an access token, which will be stored in `~/.zos/oauth_creds.json`, enabling it to interact with the Gemini API.
