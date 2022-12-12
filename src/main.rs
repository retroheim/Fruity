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
    message: Vec<String>,
    role: u64,
    server: String
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
        
        // Preparing to read
        screenshot().await;
        
        // Setting up bot
        let bot = Http::new(&config.data.token);
        let channel_id = ChannelId(config.data.channel);
        
        // Read the message from image
        let msg:String = ocr().await.to_uppercase();
        let mut count = 0usize;
        loop {
            if count < config.data.message.len() {
                count += 1;
                if msg.contains(&config.data.message[count -1]) {
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
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("images/roblox.png").unwrap();
    return lt.get_utf8_text().unwrap();
}

async fn screenshot() {
    let screen = Screen::from_point(0,0).unwrap();
    let scr = screen.capture().unwrap();
    let buffer = scr.buffer();
    fs::write("images/roblox.png", &buffer).unwrap();
    let mut img = image::open("images/roblox.png").unwrap();
    let x = screen.width / 4;
    let y = screen.height / 64;
    let width = screen.width / 2;
    let height = screen.height / 4;
    img = img.crop(x, y, width, height);
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