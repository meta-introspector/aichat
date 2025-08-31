use anyhow::Result;
use assert_cmd::Command;
use std::process::Output;

pub async fn run_aichat_login_command() -> Result<Output> {
    let mut cmd = Command::cargo_bin("aichat")?;
    let output = cmd.arg("auth").arg("login").output()?;
    Ok(output)
}
