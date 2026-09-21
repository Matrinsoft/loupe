// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]
#![doc(
    html_logo_url = "https://gitlab.gnome.org/GNOME/libgweather/-/raw/main/doc/libgweather-logo.svg",
    html_favicon_url = "https://gitlab.gnome.org/GNOME/libgweather/-/raw/main/doc/libgweather-logo.svg"
)]
//! # Rust GWeather bindings
//!
//! This library contains safe Rust bindings for [GWeather](https://gitlab.gnome.org/GNOME/libgweather/), a library that offers
//! weather data collection
//!
//! See also
//!
//! - [GWeather documentation](https://gnome.pages.gitlab.gnome.org/libgweather/)
//! - [gtk-rs project overview](https://gtk-rs.org/)
//!

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use ffi;
pub use gio;
pub use glib;

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;

mod conditions;
mod info;
mod location;

pub use auto::*;
pub use conditions::Conditions;

pub mod builders {
    pub use super::auto::builders::*;
}
