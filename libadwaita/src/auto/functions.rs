// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

#[doc(alias = "hdy_ease_out_cubic")]
pub fn ease_out_cubic(t: f64) -> f64 {
    assert_initialized_main_thread!();
    unsafe { ffi::hdy_ease_out_cubic(t) }
}

#[doc(alias = "hdy_get_enable_animations")]
pub fn get_enable_animations<P: IsA<gtk::Widget>>(widget: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::hdy_get_enable_animations(
            widget.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "hdy_init")]
pub fn init() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::hdy_init();
    }
}
