// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::HeaderBar;
use crate::HeaderGroup;
use crate::HeaderGroupChildType;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct HeaderGroupChild(Object<ffi::HdyHeaderGroupChild, ffi::HdyHeaderGroupChildClass>);

    match fn {
        get_type => || ffi::hdy_header_group_child_get_type(),
    }
}

impl HeaderGroupChild {
    #[doc(alias = "hdy_header_group_child_get_child_type")]
    pub fn get_child_type(&self) -> HeaderGroupChildType {
        unsafe {
            from_glib(ffi::hdy_header_group_child_get_child_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "hdy_header_group_child_get_gtk_header_bar")]
    pub fn get_gtk_header_bar(&self) -> Option<gtk::HeaderBar> {
        unsafe {
            from_glib_none(ffi::hdy_header_group_child_get_gtk_header_bar(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "hdy_header_group_child_get_header_bar")]
    pub fn get_header_bar(&self) -> Option<HeaderBar> {
        unsafe {
            from_glib_none(ffi::hdy_header_group_child_get_header_bar(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "hdy_header_group_child_get_header_group")]
    pub fn get_header_group(&self) -> Option<HeaderGroup> {
        unsafe {
            from_glib_none(ffi::hdy_header_group_child_get_header_group(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for HeaderGroupChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HeaderGroupChild")
    }
}
