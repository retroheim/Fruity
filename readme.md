## GPO Devil Fruit Spawn Notifier

Might be useful on other games.
This is a discord bot, that upon getting the fruit has spawned message, sends a message to a discord channel.

This does not use exploits, it just reads letters from your screen. It doesn't even know if roblox exists.
This has also been semi-allowed by a moderator from the GPO team, though that was a while ago. Not that they can detect it anyways.

Releasing it because I don't care that much about GPO and my friends stopped playing.

![Discord Screenshot](/images/Discord.png "Discord Screenshot")

### Config Variable Values:

- token: Your Discord Bot Token (with intents on)
- channel: Your channel id in numbers
- keywords: words to filter ocr messages with.
- role: Your ping role id in numbers
- server: your gpo server in the exact same characters within the quotes.
- show_warnings: boolean (true or false) variable to show warnings or not.
- crop: Divides Screen Size by these variables to locate a relative location where you want the reader to point.
- check_image_timer: Time in milliseconds, between each check for a message notification.

### Requirements:

- Basic Discord Bot Knowledge
- An All-Seeing Eye
- A computer you can run it with, since roblox needs to be in the foreground

#### How to run:

1. Modify config.json to add the proper values into the variables (they can be changed on the fly)
2. run fruity.exe
3. Launch GPO and make it fullscreen on your main monitor
4. Stand in front of a solid wall
5. Turn on the least amount of graphics possible
6. Wait
7. Profit

#### Build Instructions (windows):

1. Install vcpkg from https://github.com/microsoft/vcpkg
2. SET VCPKG_DEFAULT_TRIPLET=x64-windows
3. go into the directory of vcpkg with command prompt and do .\vcpkg install leptonica
4. SET VCPKGRS_DYNAMIC=true
5. cargo build
