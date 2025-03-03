use std::env;

use dotenv::dotenv;
use rand::Rng;
use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::process::Command;

async fn get_ip() -> String {
    //TODO: Host-Abfrage zufällig machen
    let ip_hosts = [
        "ifconfig.me",
        "icanhazip.com",
        "ipinfo.io/ip",
        "api.ipify.org",
        "ipecho.net/plain",
    ];

    let mut threadrng = rand::thread_rng();
    let random_index = threadrng.gen_range(0..ip_hosts.len());

    let element = ip_hosts[random_index];
    print!("Fetching {element}...");
    match Command::new("curl").args([element]).output() {
        Ok(output) => {
            let output = output.stdout.iter().map(|&x| x as char).collect::<String>();
            println!(" {output}");
            output
        }
        Err(e) => format!("Error: {e}"),
    }
}

// async fn interval_request_ip() {
//     let _ip = get_ip().await;

//     // get saved channel_id and saved ip
//     // check if saved ip is different from current ip
//     // if different, send message to saved channel_id
// }

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_ascii_lowercase() == "ip" {
            let ip = get_ip().await;

            match msg.channel_id.say(&ctx.http, format!("{}:25565", ip)).await {
                Ok(_) => (),
                Err(e) => println!("Error sending message: {e:?}"),
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        println!("Servers:");
        for guild in ready.guilds {
            println!(" - {}", guild.id);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Discord token is not set.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .activity(ActivityData::custom("'ip' schreiben für die IP."))
        .await
        .expect("Error creating client");

    match client.start().await {
        Ok(_) => println!("Client started successfully"),
        Err(e) => println!("Client failed to start: {e:?}"),
    }

    // let interval_task = task::spawn(async {
    //     let mut interval = time::interval(Duration::from_secs(42 * 60));
    //     loop {
    //         interval.tick().await;
    //         interval_request_ip().await;
    //     }
    // });

    // interval_task.await.expect("Failed to run async task");
}
