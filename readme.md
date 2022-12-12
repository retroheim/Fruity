### GPO Devil Fruit Spawn Notifier

Might be useful on other games.
This is a discord bot, that upon getting the fruit has spawned message, sends a message to a discord channel.

This does not use exploits, it just reads letters from your screen. It doesn't even know if roblox exists.
It is coded for different OSes, but only windows will be provided, since if you use something else, you should be able to compile it yourself.
This has also been semi-allowed by a moderator from the GPO team, though that was a while ago. Not that they can detect it anyways.

Releasing it because I don't care that much about GPO and my friends stopped playing.

Requirements:

- Basic Discord Bot Knowledge
- An All-Seeing Eye
- A computer you can run it with, since it needs to be in the foreground foreground

How to run:

1. Modify config.json to add the proper values into the variables
2. run fruity.exe
3. Launch GPO and make it fullscreen on your main monitor
4. Stand in front of a solid wall
5. Turn on the least amount of graphics possible
6. Wait
7. Profit

Build Requirements:

- Leptonica
- Tesseract
- Rust

Build Instructions:

1. View https://github.com/ccouzens/leptonica-sys
2. cargo build
