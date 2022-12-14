use std::{fs};
use serde::{Serialize, Deserialize};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use tokio::time::{Duration, sleep};
use tools::set_interval;
use tools::image_tools::{ocr, screenshot};
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
    let loop_config: String = fs::read_to_string("config.json").expect("unable to read file");
    let config: Config = serde_json::from_str(&loop_config).expect("Unable to parse json");

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
        let jsonfile = fs::read_to_string("config.json").expect("unable to read file");
        let config: Config = serde_json::from_str(&jsonfile).expect("unable to parse json");

        // Screenshots Main Display and Crops image.
        screenshot(&config.data.crop.x, &config.data.crop.y, &config.data.crop.width, &config.data.crop.height);
        
        // Read the message from Cropped Screenshotted Image
        let msg:String = ocr().await.to_uppercase();
        let mut count = 0usize;

        // Loops through all keywords to see if message contains one. Effectively a for loop.
        loop {
            if count < &config.data.keywords.len() -1 {
                count += 1;

                // Check if Read Message has a Keyword from Config File.
                if msg.contains(&config.data.keywords[count]) {
                    
                    // Setup Discord Bot
                    let bot = Http::new(&config.data.token);
                    let channel_id = ChannelId(config.data.channel);

                    // Sends Message to configured Discord Channel whilst notifying the role configured
                    let sentmessage = channel_id.send_message(&bot, |m| {
                        m.content(format!("<@&{}>", &config.data.role)).embed(|e| e.title(&msg).description(&config.data.server)).add_file("images/roblox.png")
                    }).await;
                    sentmessage.unwrap();
                    println!("{}", &msg);
                    break;
                }
            } else {
                break;
            }
        }
    }, Duration::from_millis(config.data.check_image_timer));
}