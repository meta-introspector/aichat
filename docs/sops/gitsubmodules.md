# SOP: Git Submodule Update

This Standard Operating Procedure (SOP) outlines the steps for effectively updating Git submodules within this project. Keeping submodules up-to-date is crucial for maintaining a consistent and functional development environment, especially when dependencies or external components are managed as submodules.

## Purpose

To ensure all Git submodules are correctly initialized, updated, and synchronized with their respective upstream repositories, preventing build failures, outdated dependencies, or unexpected behavior.

## Scope

This SOP applies to all developers working on projects that utilize Git submodules.

## Procedure

### 1. Navigate to the Project Root

Ensure you are in the root directory of your main Git repository.

```bash
cd /path/to/your/main/repository
```

### 2. Initialize and Update Submodules

This command initializes any new submodules and updates existing ones to the commit specified by the superproject. The `--recursive` flag ensures that nested submodules (submodules within submodules) are also initialized and updated.

```bash
bash
git submodule update --init --recursive
```

-   `--init`: Initializes new submodules found in `.gitmodules`. If a submodule is not yet initialized, this command will clone the repository into the designated path.
-   `--recursive`: Processes all nested submodules. Without this flag, only top-level submodules would be updated.

### 3. Pull Latest Changes (Optional, but Recommended for Active Development)

If you want to update a submodule to its latest commit on its default branch (e.g., `main` or `master`), you can navigate into the submodule's directory and pull the changes. After pulling, you must return to the superproject and record the new submodule commit.

```bash
# Navigate into the submodule directory
cd path/to/your/submodule

# Pull the latest changes from the remote
git pull origin <branch_name> # e.g., git pull origin main

# Return to the main repository root
cd -
```

### 4. Record Submodule Changes in the Superproject

After updating a submodule (e.g., by pulling new commits within the submodule's directory), the main repository needs to record this change. The submodule's entry in the main repository's index will show as "modified content".

```bash
git add path/to/your/submodule
```

### 5. Commit the Changes

Commit the updated submodule reference in the main repository. This ensures that other developers pulling your changes will get the correct version of the submodule.

```bash
git commit -m "Update submodule: <submodule_name>"
```

### 6. Push Changes

Push your changes to the remote repository.

```bash
git push
```

## Troubleshooting

-   **"fatal: no submodule mapping found"**: This error indicates a discrepancy between your `.gitmodules` file and Git's internal tracking. You might need to manually edit `.gitmodules` or use `git submodule deinit` and `git submodule add` to re-establish the submodule.
-   **"Submodule 'X' has uncommitted changes"**: Navigate into the submodule directory (`cd X`), commit or stash the changes, and then return to the superproject.
-   **"Submodule 'X' has modified content" but `git commit` doesn't recognize it"**: Ensure the submodule is in a clean state (`git reset --hard HEAD` within the submodule) and then explicitly `git add` the submodule path in the superproject.

## Project-Specific Submodule Information

This project utilizes the following Git submodules:

-   `oauth_review/r-google-oauth2`: Used for Google OAuth2 integration.
-   `oauth_review/rs-gapi-oauth`: Used for Google API client. (Note: This submodule recently had its URL updated to `https://github.com/meta-introspector/mass10-rs-gapi-oauth-archive` due to repository availability issues.)
-   `vendor/casbin-rs`: Casbin-RS for access control.
-   `vendor/fastly-compute-rust-auth`: Fastly Compute@Edge Rust authentication library.
-   `vendor/google-apis/gcloud-sdk-rs`: Google Cloud SDK for Rust.
-   `vendor/google-apis/google-api-rust-client`: Google API Rust client.
-   `vendor/google-apis/google-apis-rs`: Google APIs for Rust.
-   `vendor/google-apis/google-cloud-rust`: Google Cloud Rust client.
-   `vendor/google-apis/google-oauth`: Google OAuth library.
-   `vendor/google-apis/tlns-google-oauth2`: Another Google OAuth2 library.

### Known Issues and Considerations

During recent operations, the `oauth_review/rs-gapi-oauth` submodule presented challenges due to its original repository becoming unavailable. This required updating its URL and re-initializing the submodule. If you encounter issues with this specific submodule, ensure its URL in `.gitmodules` is `https://github.com/meta-introspector/mass10-rs-gapi-oauth-archive` and perform a `git submodule update --init --recursive`.

Additionally, submodules can sometimes get into a "dirty" state (e.g., uncommitted changes within the submodule's directory) which can prevent the main repository from correctly tracking their state. If `git status` reports "modified content" for a submodule but `git commit` doesn't recognize it, try the following:

1.  Navigate into the submodule directory: `cd path/to/submodule`
2.  Reset the submodule to its HEAD: `git reset --hard HEAD`
3.  Return to the main repository: `cd -`
4.  Then, `git add path/to/submodule` and `git commit`.