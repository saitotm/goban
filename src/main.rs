mod cui;
mod goban;
mod params;
mod translator;

use anyhow::Result;

fn main() -> Result<()> {
    cui::launch()?;

    Ok(())
}
