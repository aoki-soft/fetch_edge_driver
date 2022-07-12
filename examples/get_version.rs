use anyhow::Result;
use fetch_edge_driver::get_version;

fn main() -> Result<()>{
    let msedge_version = get_version()?;
    println!("{}", msedge_version);
    Ok(())
}
