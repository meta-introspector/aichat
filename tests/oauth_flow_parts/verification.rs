use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use std::process::Output;

pub fn verify_oauth_success(home_dir: &PathBuf, output: &Output) -> Result<()> {
    // Assert that the command exited successfully
    assert!(output.status.success());

    // Verify that oauth_creds.json was created
    let creds_path = home_dir.join(".zos").join("oauth_creds.json");
    assert!(fs::metadata(&creds_path).is_ok());

    // Read and verify the content of oauth_creds.json (simplified check)
    let creds_content = fs::read_to_string(&creds_path)?;
    assert!(creds_content.contains("access_token"));
    assert!(creds_content.contains("refresh_token"));

    Ok(())
}
