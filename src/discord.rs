use std::{time::{SystemTime, UNIX_EPOCH}, error::Error};

use discord_rich_presence::{DiscordIpcClient, DiscordIpc, activity::{Activity, Assets, Timestamps}};

pub struct Discord {
    client: DiscordIpcClient,
}

impl Discord {
    pub fn new(client_id: &str) -> Result<Discord, Box<dyn Error>> {
        let mut dc = Discord{
            client: DiscordIpcClient::new(client_id)?,
        };
        dc.client.connect()?;
        Ok(dc)
    }

    pub fn set_presence(&mut self, game_title: &str, image: &str) -> Result<(), Box<dyn Error>> {
        println!("Setting presence... {game_title} : {image}");
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
        self.client.set_activity(Activity::new()
            .state(game_title)
            .details(game_title)
            .assets(Assets::new()
                .large_image(image)
            )
            .timestamps(Timestamps::new()
                .start(time.as_secs() as i64)
            )
        )?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.close()
    }
}
