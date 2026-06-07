use anyhow::Result;

/// Initializes the settings.ini file with default values.
/// This binary should only run during first install of the snap.
#[tokio::main]
async fn main() -> Result<()> {
    env::init_env().await?;
    Ok(())
}