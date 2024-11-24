use std::ffi::OsStr;

use udev::{Device, Event};

pub use self::filter::DevFilter;
pub use self::filter_unique::UniqueFilter;
pub use self::filter_usbid::USBIDFilter;
pub use self::filter_usbidclass::USBHIDClass;
pub use self::filter_usbidclass::USBIDClassFilter;
pub use self::listener::DevEvent;
pub use self::listener::DevEventType;
pub use self::listener::DevListener;

mod filter;
mod filter_unique;
mod filter_usbid;
mod filter_usbidclass;
mod listener;

fn get_event_property<'a>(ev: &'a Event, key: &'a str) -> &'a str {
    ev.property_value(key)
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("")
}

fn get_device_property<'a>(ev: &'a Device, key: &'a str) -> &'a str {
    ev.property_value(key)
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("")
}
