# Plan: Create a Standalone `aichat-oauth-client` Crate

## Objective
To extract and centralize Google OAuth client creation logic into a new, reusable Rust library crate named `aichat-oauth-client`. This crate will be used by the `aichat` project and can potentially be used by other projects requiring similar functionality.

## Proposed Crate Name
`aichat-oauth-client`

## Crate Responsibilities
*   Provide a function to set up a Google OAuth 2.0 client with PKCE (Proof Key for Code Exchange) support.
*   Encapsulate the `oauth2` crate dependencies specifically for OAuth client creation.

## Detailed Steps

1.  **Create the new crate:**
    *   Create a new Rust library crate named `aichat-oauth-client` in the `clients/` directory of the `aichat` project.
    *   Command: `cargo new --lib clients/aichat-oauth-client`

2.  **Move `setup_oauth_client` function and dependencies:**
    *   Copy the `setup_oauth_client` function from `src/auth/oauth_split/oauth_client_setup.rs` to `clients/aichat-oauth-client/src/lib.rs`.
    *   Copy necessary `use` statements (`anyhow::Result`, `oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl, PkceCodeChallenge, PkceCodeVerifier}`, `crate::auth::oauth_split::constants::GoogleOAuthClient`, `oauth2::basic::BasicClient`) to `clients/aichat-oauth-client/src/lib.rs`.
    *   Adjust `GoogleOAuthClient` type if it's an alias from `aichat`'s internal types. It's likely `oauth2::basic::BasicClient` so it should be fine.
    *   Add `oauth2` and `anyhow` as dependencies to `clients/aichat-oauth-client/Cargo.toml`.

3.  **Update `aichat` project to use the new crate:**
    *   Add `aichat-oauth-client` as a dependency in `aichat`'s main `Cargo.toml`.
    *   Modify `src/auth/oauth_split/oauth_client_setup.rs` to call the `setup_oauth_client` function from the new `aichat-oauth-client` crate.
    *   Remove the original `setup_oauth_client` function and its related imports from `src/auth/oauth_split/oauth_client_setup.rs`.

4.  **Refactor for reusability (if needed):**
    *   Review the moved code in `aichat-oauth-client/src/lib.rs` to ensure it is well-structured and generic enough for its intended purpose. For now, the hardcoded Google URLs are acceptable as the primary use case is Google OAuth.

5.  **Add basic tests to the new crate:**
    *   Write a simple unit test in `clients/aichat-oauth-client/src/lib.rs` to verify that `setup_oauth_client` correctly initializes an OAuth client and generates PKCE parameters.
    *   Command to run tests: `cargo test --package aichat-oauth-client` (after creating the crate and adding the test).

## Verification
*   Ensure `aichat` builds successfully after the changes.
*   Run `aichat`'s existing OAuth-related tests to confirm no regressions.
*   Run tests for the new `aichat-oauth-client` crate.
