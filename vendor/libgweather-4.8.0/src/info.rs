// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Info;
use glib::translate::*;
use std::mem;

impl Info {
    #[doc(alias = "gweather_info_get_upcoming_moonphases")]
    #[doc(alias = "get_upcoming_moonphases")]
    pub fn upcoming_moonphases(&self) -> Option<libc::c_long> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gweather_info_get_upcoming_moonphases(
                self.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if ret { Some(value.assume_init()) } else { None }
        }
    }
}
