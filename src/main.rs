extern crate dbus;
extern crate evdev_rs as evdev;
extern crate getopts;
extern crate nix;

#[macro_use]
extern crate log;

use dbus::blocking::Connection;
use evdev::*;
use getopts::Options;
use nix::errno::Errno;
use std::fs::File;
use std::process::exit;
use std::sync::mpsc;
use std::{env, thread, time};

mod upower_kbd_backlight;
use upower_kbd_backlight::OrgFreedesktopUPowerKbdBacklight;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Because the event loop *waits* for keyboard events, we use a thread
fn spawn_input_handle(device_file: String, tx: mpsc::Sender<bool>) {
    let _ = thread::spawn(move || {
        // Open the device file (e.g. /dev/input/event1)
        let device_file = File::open(&device_file).unwrap_or_else(|e| panic!("{}", e));
        // Setup evdev
        let mut device = Device::new().unwrap();
        device.set_fd(device_file).unwrap();
        println!(
            "Input device ID: bus 0x{:x} vendor 0x{:x} product 0x{:x}",
            device.bustype(),
            device.vendor_id(),
            device.product_id()
        );
        println!("Evdev version: {:x}", device.driver_version());
        println!("Input device name: \"{}\"", device.name().unwrap_or(""));
        println!("Phys location: {}", device.phys().unwrap_or(""));
        // Events (key presses) will be stored here
        let mut event: Result<(ReadStatus, InputEvent), Errno>;
        loop {
            // Blocks until a new event is received (waits for key press)
            event = device.next_event(evdev::ReadFlag::NORMAL | evdev::ReadFlag::BLOCKING);
            if event.is_err() {
                debug!("Device event error: {:?}", event.err());
                continue;
            }
            tx.send(true).unwrap();
        }
    });
}

fn main() {
    let config = parse_args();
    debug!("Config: {:?}", config);

    // Setup messaging channel and spawn input thread
    let (tx, rx) = mpsc::channel();
    spawn_input_handle(config.device_file, tx);

    // Setup dbus
    let conn = Connection::new_system().unwrap();
    let proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower/KbdBacklight",
        time::Duration::from_millis(5000),
    );

    let mut key_event = false;
    let mut timeout = 0;
    let mut brightness = 0;
    let mut current_brightness = -1;
    loop {
        // Wait 100ms in each loop to limit CPU usage
        thread::sleep(time::Duration::from_millis(100));
        // We only care for the LAST keyboard event, if there is any.
        for msg in rx.try_iter() {
            key_event = msg;
        }
        debug!("e: {:?}, b: {:?}, t: {:?}", key_event, brightness, timeout);
        if key_event {
            brightness = config.brightness;
            timeout = config.timeout * 10;
            key_event = false;
            continue;
        }
        if timeout > 0 {
            // Count down
            timeout -= 1;
        } else {
            // Timeout is zero? Switch lights off
            brightness = 0;
        }
        if brightness != current_brightness {
            println!("Setting brightness to {}", brightness);
            proxy.set_brightness(brightness).unwrap();
            current_brightness = brightness;
        }
    }
}

#[derive(Debug)]
struct Config {
    device_file: String,
    brightness: i32,
    timeout: i32,
}

impl Config {
    fn new(device_file: String, brightness: i32, timeout: i32) -> Self {
        Config {
            device_file: device_file,
            brightness: brightness,
            timeout: timeout,
        }
    }
}

// This function unpacks cli arguments and puts them into Config
fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("d", "device", "specify the device file", "DEVICE");
    opts.optopt(
        "b",
        "brightness",
        "target keyboard brightness (1-2)",
        "BRIGHTNESS",
    );
    opts.optopt(
        "t",
        "timeout",
        "time before the bg light turns off",
        "TIMEOUT",
    );

    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!("{}", e));
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        exit(0);
    }

    let device_file = matches
        .opt_str("d")
        .unwrap_or("/dev/input/event3".to_string());

    let brightness: i32 = matches
        .opt_str("b")
        .unwrap_or("2".to_string())
        .parse()
        .unwrap();

    let timeout: i32 = matches
        .opt_str("t")
        .unwrap_or("15".to_string())
        .parse()
        .unwrap();

    Config::new(device_file, brightness, timeout)
}
