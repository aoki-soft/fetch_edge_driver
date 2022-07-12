use anyhow::Result;
use fetch_edge_driver::get_msedge_version;

fn main() -> Result<()>{
    let msedge_version = get_msedge_version()?;
    println!("{}", msedge_version);
    Ok(())
}
