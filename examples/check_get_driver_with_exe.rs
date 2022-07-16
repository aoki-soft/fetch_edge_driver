use anyhow::Result;
use fetch_edge_driver::check_get_driver_with_exe;

fn main() -> Result<()>{
    let result = check_get_driver_with_exe()?;
    println!("{:?}",result);
    Ok(())
}
