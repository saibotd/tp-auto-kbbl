# ThinkPad Auto Keyboard Backlighting (tp-auto-kbbl)

`tp-auto-kbbl` is a tool that runs in the background on your Linux laptop.
It checks for events (default `/dev/input/event3`) and enables backlighting
whenever you start typing. After typing ends + a short timeout, the backlight
turns off automatically.

To keep CPU and RAM usage low I've used [Rust](https://www.rust-lang.org/).
This is my first project in it, so if spot anything weird – or plain wrong –
let me know! I'm thankful for any advice.

## Warning

`tp-auto-kbbl` works mostly like a keylogger does, so always be sure to check the source code
and look for anything _iffy_.

## Usage

```
Usage: tp-auto-kbbl [options]

Options:
    -h, --help
                        prints this help message
    -v, --version
                        prints the version
    -d, --device DEVICE
                        specify the device file
    -b, --brightness BRIGHTNESS
                        target keyboard brightness (1-2)
    -t, --timeout TIMEOUT
                        time before the bg light turns off
```

## Installation

Download the latest [release](https://github.com/saibotd/tp-auto-kbbl/releases),
or build it yourself.

### Copy the binary

    sudo cp tp-auto-kbbl /usr/bin/

### Try it out

```
tp-auto-kbbl

# You'll probably get an error like this
thread 'main' panicked at 'Permission denied (os error 13)', src/main.rs:44:74

# Once more as root
sudo tp-auto-kbbl

Input device ID: bus 0x11 vendor 0x1 product 0x1
Evdev version: 10001
Input device name: "AT Translated Set 2 keyboard"
Phys location: isa0060/serio0/input0
Setting brightness to 0
```

Your backlight should turn off immediately. Try if it turns on when you press any button.
If not, check if the input device is correct, you may adjust it via the `-d` parameter.

### Systemd service

```
# Copy the [unit file](https://raw.githubusercontent.com/saibotd/tp-auto-kbbl/master/tp-auto-kbbl.service)
sudo cp tp-auto-kbbl.service /etc/systemd/system/

# (optional) Check if the parameters are correct
sudo nano /etc/systemd/system/tp-auto-kbbl.service

# Reload daemons
sudo systemctl daemon-reload

# Start the service
sudo systemctl start tp-auto-kbbl.service

# Check if the service runs fine
sudo systemctl status tp-auto-kbbl.service

Mär 04 14:02:06 Huffer systemd[1]: Started Auto toggle keyboard back-lighting.
Mär 04 14:02:06 Huffer tp-auto-kbbl[343]: Input device ID: bus 0x11 vendor 0x1 p
Mär 04 14:02:06 Huffer tp-auto-kbbl[343]: Evdev version: 10001
Mär 04 14:02:06 Huffer tp-auto-kbbl[343]: Input device name: "AT Translated Set
Mär 04 14:02:06 Huffer tp-auto-kbbl[343]: Phys location: isa0060/serio0/input0
Mär 04 14:02:07 Huffer tp-auto-kbbl[343]: Setting brightness to 0
Mär 04 14:02:09 Huffer tp-auto-kbbl[343]: Setting brightness to 2
Mär 04 14:02:25 Huffer tp-auto-kbbl[343]: Setting brightness to 0

# Finally enable the service
sudo systemctl enable tp-auto-kbbl.service

```

Copyright © 2020 Tobias Duehr
