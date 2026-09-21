// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Location;

use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::pin::Pin;
use std::ptr;

impl Location {
    #[doc(alias = "gweather_location_detect_nearest_city")]
    pub fn detect_nearest_city<P: FnOnce(Result<Location, glib::Error>) + 'static>(
        &self,
        lat: f64,
        lon: f64,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn detect_nearest_city_trampoline<
            P: FnOnce(Result<Location, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            unsafe {
                let mut error = ptr::null_mut();
                let ret = ffi::gweather_location_detect_nearest_city_finish(res, &mut error);
                let result = if error.is_null() {
                    Ok(from_glib_full(ret))
                } else {
                    Err(from_glib_full(error))
                };
                let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                    Box_::from_raw(user_data as *mut _);
                let callback: P = callback.into_inner();
                callback(result);
            }
        }
        let callback = detect_nearest_city_trampoline::<P>;
        unsafe {
            ffi::gweather_location_detect_nearest_city(
                self.to_glib_none().0,
                lat,
                lon,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn detect_nearest_city_future(
        &self,
        lat: f64,
        lon: f64,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Location, glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.detect_nearest_city(lat, lon, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}
