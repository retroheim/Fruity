use std::{fs};
use image::{ImageFormat};
use serde::{Serialize, Deserialize};
use std::future::Future;
use serenity::{async_trait};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use tokio::time::{self, Duration, sleep};
use screenshots::Screen;

struct Handler;
#[derive(Serialize, Deserialize)]
struct Data {
    token: String,
    channel: u64,
    keywords: Vec<String>,
    role: u64,
    server: String,
    show_warnings: bool
}
#[derive(Serialize, Deserialize)]
struct Config {
    data: Data
}

#[async_trait]
impl EventHandler for Handler {}
#[tokio::main]
async fn main() {
    startup().await;
    // This is to stop the program from closing
    sleep(Duration::from_secs(999999999)).await;
}

async fn startup() {
    set_interval(|| async {
        // Reading the configs
        let jsonfile = fs::read_to_string("config.json").expect("unable to read file");
        let config: Config = serde_json::from_str(&jsonfile).expect("unable to parse json");

        if config.data.show_warnings {
            if config.data.token.is_empty() {println!("{}", "Warning 1: NO DISCORD BOT TOKEN CONFIGURED!!!")};
            if config.data.channel < 10000000000000000 {println!("{}", "Warning 2: Channel ID Improperly Configured!")};
            if config.data.role < 10000000000000000 {println!("{}", "Warning 3: Role ID Improperly Configured!")};
            if config.data.keywords.len() < 2 {println!("{}", "Warning 4: Not Enough Keywords!")};
            if config.data.server.is_empty() {println!("{}", "Warning 5: Nothing Configured for Server Config")};
        }
        // Screenshot Image to be Read.
        screenshot().await;
        
        // Setup Discord Bot
        let bot = Http::new(&config.data.token);
        let channel_id = ChannelId(config.data.channel);
        
        // Read the message from Cropped Screenshotted Image
        let msg:String = ocr().await.to_uppercase();
        let mut count = 0usize;
        loop {
            if count < config.data.keywords.len() {
                count += 1;

                // Check if Read Message has a Keyword from Config File.
                if msg.contains(&config.data.keywords[count -1]) {
                    // Send Successful Keyword Detected Message to configured Discord Channel whilst notifying the role configured
                    let sentmessage = channel_id.send_message(&bot, |m| {
                        m.content(format!("<@&{}>", config.data.role)).embed(|e| e.title(&msg).description(config.data.server)).add_file("images/roblox.png")
                    }).await;
                    sentmessage.unwrap();
                    println!("{}", &msg);
                    break;
                }
            } else {
                println!("{}", &msg);
                break;
            }
        }
    }, Duration::from_millis(8500));
}

async fn ocr() -> String {
    // Prepare Optical Character Recognition
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();

    // Set Cropped ScreenShotted Image to be Read.
    lt.set_image("images/roblox.png").unwrap();

    // Return Read Characters
    return lt.get_utf8_text().unwrap();
}

async fn screenshot() {
    // Take Screenshot of Main Monitor
    let screen = Screen::from_point(0,0).unwrap();
    let scr = screen.capture().unwrap();
    let buffer = scr.buffer();

    // Write Screenshot to roblox.png
    fs::write("images/roblox.png", &buffer).unwrap();

    // Re-Open Screenshot as Image
    let mut img = image::open("images/roblox.png").unwrap();

    // Crop Image to a good Size (for Better Accuracy)
    let x = screen.width / 4;
    let y = screen.height / 64;
    let width = screen.width / 2;
    let height = screen.height / 4;
    img = img.crop(x, y, width, height);

    // Overwrite Save Image
    let path = format!("images/roblox.png");
    img
        .save_with_format(&path, ImageFormat::Png)
        .unwrap();
}

fn set_interval<F, Fut>(mut f: F, dur: Duration)
where
    F: Send + 'static + FnMut() -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    // Create stream of intervals.
    let mut interval = time::interval(dur);
    
    tokio::spawn(async move {
        // Skip the first tick at 0ms.
        interval.tick().await;
        loop {
            // Wait until next tick.
            interval.tick().await;
            // Spawn a task for this tick.
            tokio::spawn(f());
        }
    });
}