use std::{process::Command};
use anyhow::{Result, Context};
use bytes::Bytes;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
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

pub fn get_driver<P: AsRef<Path>>(driver_path: P) -> Result<()> {
    let edge_version = get_version()?;
    let data = download_driver(&edge_version)?;
    let cur = Cursor::new(&data);
    unzip_driver(cur, driver_path)?;
    Ok(())
}

pub fn unzip_driver<P: AsRef<Path>>(data: impl Read + Seek, driver_path: P) -> Result<()> {
    let mut zip_data = zip::ZipArchive::new(data)?;
    let mut zip_driver = zip_data.by_name("msedgedriver.exe")?;
    let mut driver_data = Vec::new();
    
    zip_driver.read_to_end(&mut driver_data)?;

    let mut file = std::fs::File::create(driver_path)?;
    file.write_all(&driver_data)?;
    Ok(())
}

pub fn save_driver_with_exe() -> Result<PathBuf> {
    let mut driver_path = std::env::current_exe()?;
    driver_path.pop();
    driver_path.push("msedgedriver.exe");

    get_driver(&driver_path)?;
    Ok(driver_path)
}