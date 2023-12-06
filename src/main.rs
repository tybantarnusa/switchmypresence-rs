mod discord;

use std::error::Error;
use crate::discord::Discord;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dc = Discord::new("848100876773621771")?;
    dc.set_presence("Test Game", "octopath")?;

    loop {}

    dc.close()?;
    Ok(())
}
