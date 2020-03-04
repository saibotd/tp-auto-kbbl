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

fn main() {
    let config = parse_args();
    debug!("Config: {:?}", config);

    let device_file = File::open(&config.device_file).unwrap_or_else(|e| panic!("{}", e));
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

    let mut event: Result<(ReadStatus, InputEvent), Errno>;

    let (tx, rx) = mpsc::channel();
    let _ = thread::spawn(move || {
        let conn = Connection::new_system().unwrap();
        let proxy = conn.with_proxy(
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower/KbdBacklight",
            time::Duration::from_millis(5000),
        );
        let default_val: Box<[i32]> = Box::new([-1, -1]);
        let mut current_brightness: i32 = -1;
        let mut val = default_val.clone();
        loop {
            thread::sleep(time::Duration::from_millis(100));
            // We only care for the LAST submitted action
            for _val in rx.try_iter() {
                val = _val;
            }
            debug!("Action value: {:?}", val);
            if val[0] < 0 {
                // Brightness level below zero? Do nothing.
                continue;
            }
            if val[1] == 0 && current_brightness != val[0] {
                // timeout is zero? time to act!
                println!("Setting brightness to {}", val[0]);
                current_brightness = val[0];
                let _ = proxy.set_brightness(val[0]);
                val = default_val.clone();
            } else if val[1] > 0 {
                // Count down
                val[1] -= 1;
            }
        }
    });

    tx.send(Box::new([0, 0])).unwrap();
    loop {
        event = device.next_event(evdev::ReadFlag::NORMAL | evdev::ReadFlag::BLOCKING);
        if event.is_err() {
            debug!("Device event error: {:?}", event.err());
            continue;
        }
        tx.send(Box::new([config.brightness, 0])).unwrap();
        thread::sleep(time::Duration::from_millis(100));
        tx.send(Box::new([0, config.timeout * 10])).unwrap();
    }
}

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
