## CRQ 2: Feature Development - Implement Google Cloud OAuth Scope Management Meta Client

*   **Problem:** Managing Google Cloud OAuth "Authorized redirect URIs" and other client configurations currently requires manual interaction with the Google Cloud Console. This is cumbersome and does not align with a CLI-first approach.
*   **Proposed Change:**
    *   **New CLI Command:** Introduce a new top-level command (e.g., `aichat auth manage-google-oauth` or `aichat google-cloud oauth-config`).
    *   **"Meta Client" OAuth Flow:** This new command will initiate a separate OAuth flow using a *different* Google Cloud OAuth client ID (the "meta client"). This meta client will require specific Google Cloud scopes (e.g., `https://www.googleapis.com/auth/cloud-platform`) to manage project resources.
    *   **Programmatic Configuration Management:** Once authenticated, the meta client will allow the user to:
        *   List existing OAuth 2.0 client IDs and their configurations within a specified Google Cloud Project.
        *   Add or remove "Authorized redirect URIs" for existing OAuth 2.0 client IDs.
        *   Potentially manage other aspects of OAuth 2.0 client configurations (e.g., "Authorized JavaScript origins").
    *   **Secure Credential Storage:** The meta client's credentials will be stored securely, separate from the primary application client's credentials.
*   **Justification:** Provides a powerful, programmatic way for users to manage their Google Cloud OAuth configurations directly from the CLI, improving developer workflow and automation capabilities.
*   **Affected Files (Initial Assessment - will expand during design):**
    *   `src/cli.rs` (for new command definition)
    *   New modules under `src/auth/oauth_split/` or a new `src/google_cloud/` directory for meta client logic.
    *   `src/config/mod.rs` (for meta client configuration)
    *   New test files for the meta client functionality.
*   **Potential Google Cloud Rust SDKs/Clients for Integration:**
    1.  `https://github.com/oxidecomputer/third-party-api-clients`
    2.  `https://github.com/googleapis/google-cloud-rust/tree/main`
    3.  `https://github.com/0Itsuki0/google-api-rust-client`
    4.  `https://github.com/Byron/google-apis-rs`
    5.  `https://github.com/googleapis/google-cloud-rust`
    6.  `https://github.com/abdolence/gcloud-sdk-rs`

## Detailed Implementation Plan

A detailed Standard Operating Procedure (SOP) for the implementation of this Meta Client has been documented in `docs/sops/implementing_meta_client.md`.
