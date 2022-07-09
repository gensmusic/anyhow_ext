use anyhow_ext::{Context, Result};

fn foo() -> Result<()> {
    std::fs::read_to_string("file_not_exists.txt").dot()?;
    Ok(())
}

fn bar() -> Result<()> {
    foo().dot()?;
    Ok(())
}

fn main() {
    if let Err(err) = try_main().dot() {
        println!("{:?}", err);
    }
}

fn try_main() -> Result<()> {
    bar().dot()?;
    Ok(())
}
