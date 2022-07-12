use anyhow::Result;
use fetch_edge_driver::save_driver_with_exe;

fn main() -> Result<()>{
    save_driver_with_exe()?;
    Ok(())
}