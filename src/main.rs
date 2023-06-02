use connect4_multiplayer::{input, server, client};

#[tokio::main]
async fn main() {
    
    loop {
        print!("Host a server (y/n) ");
        match input().as_str() {
            "y" => {
                server::start().await;
                break;
            }
            "n" => {
                client::start().await;
            }
            _ => ()
        }
    }
}
