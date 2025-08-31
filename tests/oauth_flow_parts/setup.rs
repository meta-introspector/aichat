use anyhow::Result;
use tempfile::{tempdir, TempDir};
use std::path::PathBuf;

pub fn setup_test_environment() -> Result<(TempDir, PathBuf)> {
    let temp_dir = tempdir()?;
    let home_dir = temp_dir.path().to_path_buf();

    // Set HOME environment variable for the test to use a temporary directory
    // This ensures that oauth_creds.json is written to a controlled location
    std::env::set_var("HOME", &home_dir);

    Ok((temp_dir, home_dir))
}
