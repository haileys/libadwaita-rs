// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct StatusPage(Object<ffi::HdyStatusPage, ffi::HdyStatusPageClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        get_type => || ffi::hdy_status_page_get_type(),
    }
}

impl StatusPage {
    #[doc(alias = "hdy_status_page_new")]
    pub fn new() -> StatusPage {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::hdy_status_page_new()).unsafe_cast() }
    }

    #[doc(alias = "hdy_status_page_get_child")]
    pub fn get_child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::hdy_status_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "hdy_status_page_get_description")]
    pub fn get_description(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::hdy_status_page_get_description(self.to_glib_none().0)) }
    }

    #[doc(alias = "hdy_status_page_get_icon_name")]
    pub fn get_icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::hdy_status_page_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "hdy_status_page_get_title")]
    pub fn get_title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::hdy_status_page_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "hdy_status_page_set_child")]
    pub fn set_child<P: IsA<gtk::Widget>>(&self, child: Option<&P>) {
        unsafe {
            ffi::hdy_status_page_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "hdy_status_page_set_description")]
    pub fn set_description(&self, description: Option<&str>) {
        unsafe {
            ffi::hdy_status_page_set_description(
                self.to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "hdy_status_page_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::hdy_status_page_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "hdy_status_page_set_title")]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::hdy_status_page_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn connect_property_child_notify<F: Fn(&StatusPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::HdyStatusPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_description_notify<F: Fn(&StatusPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::HdyStatusPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_icon_name_notify<F: Fn(&StatusPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::HdyStatusPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_notify<F: Fn(&StatusPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::HdyStatusPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for StatusPage {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StatusPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StatusPage")
    }
}
