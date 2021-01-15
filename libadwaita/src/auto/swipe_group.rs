// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Swipeable;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct SwipeGroup(Object<ffi::HdySwipeGroup, ffi::HdySwipeGroupClass>) @implements gtk::Buildable;

    match fn {
        get_type => || ffi::hdy_swipe_group_get_type(),
    }
}

impl SwipeGroup {
    #[doc(alias = "hdy_swipe_group_new")]
    pub fn new() -> SwipeGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::hdy_swipe_group_new()) }
    }

    #[doc(alias = "hdy_swipe_group_add_swipeable")]
    pub fn add_swipeable<P: IsA<Swipeable>>(&self, swipeable: &P) {
        unsafe {
            ffi::hdy_swipe_group_add_swipeable(
                self.to_glib_none().0,
                swipeable.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "hdy_swipe_group_get_swipeables")]
    pub fn get_swipeables(&self) -> Vec<Swipeable> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::hdy_swipe_group_get_swipeables(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "hdy_swipe_group_remove_swipeable")]
    pub fn remove_swipeable<P: IsA<Swipeable>>(&self, swipeable: &P) {
        unsafe {
            ffi::hdy_swipe_group_remove_swipeable(
                self.to_glib_none().0,
                swipeable.as_ref().to_glib_none().0,
            );
        }
    }
}

impl Default for SwipeGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SwipeGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SwipeGroup")
    }
}
