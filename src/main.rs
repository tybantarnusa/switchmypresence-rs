mod discord;
use discord_rpc_client::Client;

fn main() {
    const CLIENT_ID: u64 = 848100876773621771;
    let mut drpc = Client::new(CLIENT_ID);
    drpc.start();
    discord::set_presence(&mut drpc, "Test Game".to_owned(), "octopath".to_owned());
    loop {
        println!("active...");
    }
}
