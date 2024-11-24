use udev::{Device, Event};

use crate::udevdetect::{get_device_property, get_event_property, DevFilter};

#[derive(Debug)]
pub struct BusFilter {
    bus: String,
    device_group: String,
}
impl BusFilter {
    pub fn new(bus: String, device_group: String) -> Self {
        let f = BusFilter { bus, device_group };
        debug!("New Bus Filter for: {:?} at {:?}", f.bus, f.device_group);
        f
    }
}
impl DevFilter for BusFilter {
    fn match_event(&self, e: &Event) -> bool {
        if get_event_property(e, "ID_BUS") != self.bus {
            return false;
        }
        if get_event_property(e, "LIBINPUT_DEVICE_GROUP") != self.device_group {
            return false;
        }
        true
    }

    fn match_device(&self, e: &Device) -> bool {
        if get_device_property(e, "ID_BUS") != self.bus {
            return false;
        }
        if get_device_property(e, "LIBINPUT_DEVICE_GROUP") != self.device_group {
            return false;
        }
        true
    }
}
