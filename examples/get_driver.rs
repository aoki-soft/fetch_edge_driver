use anyhow::Result;
use fetch_edge_driver::get_driver;
use std::path::Path;

fn main() -> Result<()>{
    get_driver(Path::new("msedgedriver.exe"))?;
    Ok(())
}