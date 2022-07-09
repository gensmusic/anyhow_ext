use anyhow::{anyhow, Result};
use anyhow_ext::Context;

fn foo() -> Result<()> {
    Err(anyhow!("an anyhow err")).context("wrap with file info")?;
    Ok(())
}

fn main() {
    if let Err(err) = foo() {
        println!("{:?}", err);
    }
}
