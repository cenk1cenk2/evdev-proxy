use udev::{Device, Event};

use crate::udevdetect::{get_device_property, get_event_property, DevFilter};

#[derive(Debug)]
pub struct UniqueFilter {
    unique: String,
}
impl UniqueFilter {
    pub fn new(unique: String) -> Self {
        let f = UniqueFilter { unique: unique };
        debug!("New Unique Filter for: {:?}", f.unique);
        f
    }
}
impl DevFilter for UniqueFilter {
    fn match_event(&self, e: &Event) -> bool {
        if get_event_property(e, "UNIQ") != self.unique {
            return false;
        }
        true
    }

    fn match_device(&self, e: &Device) -> bool {
        if get_device_property(e, "UNIQ") != self.unique {
            return false;
        }
        true
    }
}
