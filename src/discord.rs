use discord_rpc_client::Client;

pub fn set_presence(client: &mut Client, game_title: String, image: String) {
    println!("Setting presence... {game_title} : {image}");
    if client.set_activity(|act| {
        act.assets(|assets| {
            assets.large_image(image)
        }).state(game_title)
    }).is_err() {
        eprintln!("ERROR: Cannot set presence.");
    }
}