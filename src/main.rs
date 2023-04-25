use std::fs;
use serde::{Serialize, Deserialize};
use serenity::{http::Http, model::id::ChannelId};
use tokio::time::{Duration, sleep};
use tools::{set_interval, image_tools::{ocr, screenshot}};
pub mod tools;

#[derive(Serialize, Deserialize)]
struct Crop {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

#[derive(Serialize, Deserialize)]
struct Data {
    token: String,
    channel: u64,
    keywords: Vec<String>,
    role: u64,
    server: String,
    show_warnings: bool,
    crop: Crop,
    check_image_timer: u64
}

#[derive(Serialize, Deserialize)]
struct Config {
    data: Data
}

#[tokio::main]
async fn main() {
    startup().await;

    // This is to stop the program from closing since the discord bot itself is only on when absolutely necessary.
    sleep(Duration::from_secs(999999999)).await;
}

async fn startup() {

    // Reading Configs for Startup Configurations and Warnings
    let config: Config = fs::read_to_string("config.json")
        .and_then(|s| serde_json::from_str(&s))
        .expect("Unable to read and parse config.json");

    // Checks to see if warnings are turned on
    if config.data.show_warnings {

        // Warns User for every improperly configured config
        if config.data.token.is_empty() {println!("{}", "Warning 0: NO DISCORD BOT TOKEN CONFIGURED!")};
        if config.data.channel < 10000000000000000 {println!("{}", "Warning 1: Channel ID Improperly Configured!")};
        if config.data.role < 10000000000000000 {println!("{}", "Warning 2: Role ID Improperly Configured!")};
        if config.data.keywords.len() < 2 {println!("{}", "Warning 3: Not Enough Keywords!")};
        if config.data.server.is_empty() {println!("{}", "Warning 4: Nothing Configured for Server Config")};
        if config.data.check_image_timer > 1000 {println!("{}", "Warning 5: Image Timer Too Short!")};
    }
    
    // Starts up the auto-notification checker
    set_interval(|| async {

        // Reading the configs
        let config: Config = fs::read_to_string("config.json")
        .and_then(|s| serde_json::from_str(&s))
        .expect("Unable to read and parse config.json");

        // Screenshots Main Display and Crops image.
        let image = screenshot(&config.data.crop.x, &config.data.crop.y, &config.data.crop.width, &config.data.crop.height);
        
        // Read the message from Cropped Screenshotted Image
        let msg:String = ocr(image).await.to_uppercase();
        let mut count = 0usize;

        // Loops through all keywords to see if message contains one. Effectively a for loop.
        for keyword in &config.data.keywords {
            if msg.contains(keyword) {
                let bot = Http::new(&config.data.token);
                let channel_id = ChannelId(config.data.channel);

                let sentmessage = channel_id.send_message(&bot, |m| {
                    m.content(format!("<@&{}>", &config.data.role))
                     .embed(|e| e.title(&msg).description(&config.data.server))
                     .add_file(image)
                }).await;

                sentmessage.unwrap();
                println!("{}", &msg);
                break;
            }
        }
    }, Duration::from_millis(config.data.check_image_timer));
}