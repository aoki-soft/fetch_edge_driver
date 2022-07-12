use std::process::Command;
use anyhow::{Result, Context};

/// # Get microsoft edge version
pub fn get_msedge_version() -> Result<String> {
    let output = Command::new("powershell")
        .args(["/C", r#"(get-item ($env:SystemDrive + "\Program Files (x86)\Microsoft\Edge\Application\msedge.exe")).VersionInfo.FileVersion"#])
        .output().context("Failed get edege version cmd")?;
    let edge_version = output.stdout;
    Ok(std::str::from_utf8(&edge_version).context("Failed convert edge_version")?.trim().to_string())
}

