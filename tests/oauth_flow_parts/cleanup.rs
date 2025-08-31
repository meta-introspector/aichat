use anyhow::Result;
use tempfile::TempDir;

pub fn cleanup_test_environment(temp_dir: TempDir) -> Result<()> {
    // Clean up temporary directory
    temp_dir.close()?;
    Ok(())
}
