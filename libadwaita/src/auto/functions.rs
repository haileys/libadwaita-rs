// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

#[doc(alias = "adw_ease_out_cubic")]
pub fn ease_out_cubic(t: f64) -> f64 {
    assert_initialized_main_thread!();
    unsafe { ffi::adw_ease_out_cubic(t) }
}

#[doc(alias = "adw_get_enable_animations")]
#[doc(alias = "get_enable_animations")]
pub fn is_animations_enabled<P: IsA<gtk::Widget>>(widget: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::adw_get_enable_animations(
            widget.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "adw_init")]
pub fn init() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::adw_init();
    }
}

#[doc(alias = "adw_is_initialized")]
pub fn is_initialized() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::adw_is_initialized()) }
}
