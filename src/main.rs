mod goban;
mod translator;
mod params;
mod cui;

use anyhow::Result;

fn main() -> Result<()> {
    cui::launch()?;

    Ok(())
}

