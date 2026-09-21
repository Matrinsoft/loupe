// Take a look at the license at the top of the repository in the LICENSE file.

use std::marker::PhantomData;

use glib::translate::*;

use crate::{ConditionPhenomenon, ConditionQualifier, FormatOptions, ffi};

#[derive(Copy, Clone)]
#[doc(alias = "GWeatherConditions")]
#[repr(transparent)]
pub struct Conditions(ffi::GWeatherConditions);

impl Conditions {
    #[inline]
    pub fn new(
        significant: bool,
        phenomenon: ConditionPhenomenon,
        qualifier: ConditionQualifier,
    ) -> Self {
        Self(ffi::GWeatherConditions {
            significant: significant.into_glib(),
            phenomenon: phenomenon.into_glib(),
            qualifier: qualifier.into_glib(),
        })
    }

    #[inline]
    pub fn significant(&self) -> bool {
        unsafe { from_glib(self.0.significant) }
    }

    #[inline]
    pub fn phenomenon(&self) -> ConditionPhenomenon {
        unsafe { from_glib(self.0.phenomenon) }
    }

    #[inline]
    pub fn qualifier(&self) -> ConditionQualifier {
        unsafe { from_glib(self.0.qualifier) }
    }

    #[doc(alias = "gweather_conditions_to_string")]
    #[doc(alias = "to_string")]
    pub fn as_str(&self) -> &glib::GStr {
        unsafe {
            glib::GStr::from_ptr(ffi::gweather_conditions_to_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gweather_conditions_to_string_full")]
    #[doc(alias = "to_string_full")]
    pub fn as_str_full(&self, options: &FormatOptions) -> &glib::GStr {
        unsafe {
            glib::GStr::from_ptr(ffi::gweather_conditions_to_string_full(
                mut_override(self.to_glib_none().0),
                options.into_glib(),
            ))
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GWeatherConditions> for Conditions {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GWeatherConditions, Self> {
        Stash(
            self as *const Conditions as *const ffi::GWeatherConditions,
            PhantomData,
        )
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GWeatherConditions> for Conditions {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GWeatherConditions, Self> {
        StashMut(
            self as *mut Conditions as *mut ffi::GWeatherConditions,
            PhantomData,
        )
    }
}

impl std::fmt::Debug for Conditions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Conditions")
            .field("significant", &self.significant())
            .field("phenomenon", &self.phenomenon())
            .field("qualifier", &self.qualifier())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditions() {
        let phenomenon = crate::ConditionPhenomenon::None;
        let qualifier = crate::ConditionQualifier::None;
        let significant = true;
        let cond = Conditions::new(significant, phenomenon, qualifier);

        assert_eq!(cond.significant(), significant);
        assert_eq!(cond.qualifier(), qualifier);
        assert_eq!(cond.phenomenon(), phenomenon);

        let format = FormatOptions::SENTENCE_CAPITALIZATION;
        assert_eq!(cond.as_str(), "??");
        assert_eq!(cond.as_str_full(&format), "??");

        let phenomenon = crate::ConditionPhenomenon::Fog;
        let qualifier = crate::ConditionQualifier::Vicinity;
        let significant = true;
        let cond = Conditions::new(significant, phenomenon, qualifier);

        assert_eq!(cond.as_str(), "Fog in the vicinity");
        assert_eq!(cond.as_str_full(&format), "Fog in the vicinity");

        let phenomenon = crate::ConditionPhenomenon::Fog;
        let qualifier = crate::ConditionQualifier::Vicinity;
        let significant = false;
        let cond = Conditions::new(significant, phenomenon, qualifier);

        assert_eq!(cond.as_str(), "-");
        assert_eq!(cond.as_str_full(&format), "-");
    }
}
