// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct LeafletPage(Object<ffi::AdwLeafletPage, ffi::AdwLeafletPageClass>);

    match fn {
        type_ => || ffi::adw_leaflet_page_get_type(),
    }
}

impl LeafletPage {
    #[doc(alias = "adw_leaflet_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_leaflet_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_leaflet_page_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_leaflet_page_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_leaflet_page_get_navigatable")]
    #[doc(alias = "get_navigatable")]
    pub fn is_navigatable(&self) -> bool {
        unsafe { from_glib(ffi::adw_leaflet_page_get_navigatable(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_leaflet_page_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::adw_leaflet_page_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_leaflet_page_set_navigatable")]
    pub fn set_navigatable(&self, navigatable: bool) {
        unsafe {
            ffi::adw_leaflet_page_set_navigatable(self.to_glib_none().0, navigatable.into_glib());
        }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&LeafletPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&LeafletPage) + 'static>(
            this: *mut ffi::AdwLeafletPage,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "navigatable")]
    pub fn connect_navigatable_notify<F: Fn(&LeafletPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_navigatable_trampoline<F: Fn(&LeafletPage) + 'static>(
            this: *mut ffi::AdwLeafletPage,
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
                b"notify::navigatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_navigatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`LeafletPage`].
pub struct LeafletPageBuilder {
    child: Option<gtk::Widget>,
    name: Option<String>,
    navigatable: Option<bool>,
}

impl LeafletPageBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`LeafletPageBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`LeafletPage`].
    pub fn build(self) -> LeafletPage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref navigatable) = self.navigatable {
            properties.push(("navigatable", navigatable));
        }
        glib::Object::new::<LeafletPage>(&properties)
            .expect("Failed to create an instance of LeafletPage")
    }

    pub fn child<P: IsA<gtk::Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn navigatable(mut self, navigatable: bool) -> Self {
        self.navigatable = Some(navigatable);
        self
    }
}

impl fmt::Display for LeafletPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LeafletPage")
    }
}
