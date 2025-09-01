## Rationale for Client Architecture Changes

The recent changes to the client architecture, particularly concerning prompts and macros, were implemented to enhance flexibility, consistency, and integrate the new authentication system across all clients.

### Prompts
Modifications to `PROMPTS` constants and their handling centralize the definition of client-specific configuration prompts. This provides a unified mechanism for consistent prompt generation across different clients, simplifying management and supporting a smoother user experience, especially for authentication-related inputs.

### Macros
The `register_client!`, `impl_client_trait!`, and `config_get_fn!` macros were updated to support the new architecture and the `Authenticator` trait:
*   **Public Visibility:** Modules and functions are now generated as `pub` for better internal organization and accessibility.
*   **Authenticator Integration:** An `authenticator` argument was added to client initialization functions and propagated through macros. This allows clients to receive an `Authenticator` instance, decoupling core client logic from specific authentication methods.
*   **Asynchronous Operations:** `prepare_*` functions were made `async`, and macro calls were updated to `await` them, enabling non-blocking I/O for network-bound operations.