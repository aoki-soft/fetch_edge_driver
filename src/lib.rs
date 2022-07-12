use std::{process::Command};
use anyhow::{Result, Context};
use bytes::Bytes;
use std::io::prelude::*;
use std::path::Path;
use std::io::Cursor;

/// # Get microsoft edge version
pub fn get_version() -> Result<String> {
    let output = Command::new("powershell")
        .args(["/C", r#"(get-item ($env:SystemDrive + "\Program Files (x86)\Microsoft\Edge\Application\msedge.exe")).VersionInfo.FileVersion"#])
        .output().context("Failed get edege version cmd")?;
    let edge_version = output.stdout;
    Ok(std::str::from_utf8(&edge_version).context("Failed convert edge_version")?.trim().to_string())
}

pub fn download_driver(edge_version: &str) -> Result<Bytes> {
    let driver_url = format!("https://msedgedriver.azureedge.net/{}/edgedriver_win64.zip", edge_version);
    reqwest::blocking::get(driver_url)
        .context("Failed download")?
        .bytes()
        .context("Failed body to Bytes")
}

pub fn get_driver(dist_path: &'static Path) -> Result<()> {
    let edge_version = get_version()?;
    let data = download_driver(&edge_version)?;
    let cur = Cursor::new(&data);
    unzip_driver(cur, dist_path)?;
    Ok(())
}

pub fn unzip_driver(data: impl Read + Seek, dist_path: &'static Path) -> Result<()> {
    let mut zip_data = zip::ZipArchive::new(data)?;
    zip_data.extract(dist_path)?;
    Ok(())
}