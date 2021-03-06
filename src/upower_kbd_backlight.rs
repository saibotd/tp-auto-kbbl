// This code was autogenerated with `dbus-codegen-rust -c blocking -m None -s -g -d org.freedesktop.UPower -p /org/freedesktop/UPower/KbdBacklight -f org.freedesktop.UPower.KbdBacklight`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopUPowerKbdBacklight {
    fn get_max_brightness(&self) -> Result<i32, dbus::Error>;
    fn get_brightness(&self) -> Result<i32, dbus::Error>;
    fn set_brightness(&self, value: i32) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopUPowerKbdBacklight for blocking::Proxy<'a, C> {

    fn get_max_brightness(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.freedesktop.UPower.KbdBacklight", "GetMaxBrightness", ())
            .and_then(|r: (i32, )| Ok(r.0, ))
    }

    fn get_brightness(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.freedesktop.UPower.KbdBacklight", "GetBrightness", ())
            .and_then(|r: (i32, )| Ok(r.0, ))
    }

    fn set_brightness(&self, value: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.UPower.KbdBacklight", "SetBrightness", (value, ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopUPowerKbdBacklightBrightnessChanged {
    pub value: i32,
}

impl arg::AppendAll for OrgFreedesktopUPowerKbdBacklightBrightnessChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.value, i);
    }
}

impl arg::ReadAll for OrgFreedesktopUPowerKbdBacklightBrightnessChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopUPowerKbdBacklightBrightnessChanged {
            value: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopUPowerKbdBacklightBrightnessChanged {
    const NAME: &'static str = "BrightnessChanged";
    const INTERFACE: &'static str = "org.freedesktop.UPower.KbdBacklight";
}

#[derive(Debug)]
pub struct OrgFreedesktopUPowerKbdBacklightBrightnessChangedWithSource {
    pub value: i32,
    pub source: String,
}

impl arg::AppendAll for OrgFreedesktopUPowerKbdBacklightBrightnessChangedWithSource {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.value, i);
        arg::RefArg::append(&self.source, i);
    }
}

impl arg::ReadAll for OrgFreedesktopUPowerKbdBacklightBrightnessChangedWithSource {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopUPowerKbdBacklightBrightnessChangedWithSource {
            value: i.read()?,
            source: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopUPowerKbdBacklightBrightnessChangedWithSource {
    const NAME: &'static str = "BrightnessChangedWithSource";
    const INTERFACE: &'static str = "org.freedesktop.UPower.KbdBacklight";
}
