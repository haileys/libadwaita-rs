// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AdwSplitButton")]
    pub struct SplitButton(Object<ffi::AdwSplitButton, ffi::AdwSplitButtonClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::adw_split_button_get_type(),
    }
}

impl SplitButton {
    #[doc(alias = "adw_split_button_new")]
    pub fn new() -> SplitButton {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_split_button_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SplitButton`] objects.
    ///
    /// This method returns an instance of [`SplitButtonBuilder`](crate::builders::SplitButtonBuilder) which can be used to create [`SplitButton`] objects.
    pub fn builder() -> SplitButtonBuilder {
        SplitButtonBuilder::new()
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "adw_split_button_get_can_shrink")]
    #[doc(alias = "get_can_shrink")]
    pub fn can_shrink(&self) -> bool {
        unsafe { from_glib(ffi::adw_split_button_get_can_shrink(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_split_button_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_split_button_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_split_button_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self) -> gtk::ArrowType {
        unsafe { from_glib(ffi::adw_split_button_get_direction(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_split_button_get_dropdown_tooltip")]
    #[doc(alias = "get_dropdown_tooltip")]
    pub fn dropdown_tooltip(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::adw_split_button_get_dropdown_tooltip(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_split_button_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_split_button_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_split_button_get_label")]
    #[doc(alias = "get_label")]
    pub fn label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_split_button_get_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_split_button_get_menu_model")]
    #[doc(alias = "get_menu_model")]
    pub fn menu_model(&self) -> Option<gio::MenuModel> {
        unsafe { from_glib_none(ffi::adw_split_button_get_menu_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_split_button_get_popover")]
    #[doc(alias = "get_popover")]
    pub fn popover(&self) -> Option<gtk::Popover> {
        unsafe { from_glib_none(ffi::adw_split_button_get_popover(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_split_button_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    pub fn uses_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_split_button_get_use_underline(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_split_button_popdown")]
    pub fn popdown(&self) {
        unsafe {
            ffi::adw_split_button_popdown(self.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_split_button_popup")]
    pub fn popup(&self) {
        unsafe {
            ffi::adw_split_button_popup(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "adw_split_button_set_can_shrink")]
    pub fn set_can_shrink(&self, can_shrink: bool) {
        unsafe {
            ffi::adw_split_button_set_can_shrink(self.to_glib_none().0, can_shrink.into_glib());
        }
    }

    #[doc(alias = "adw_split_button_set_child")]
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_split_button_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_split_button_set_direction")]
    pub fn set_direction(&self, direction: gtk::ArrowType) {
        unsafe {
            ffi::adw_split_button_set_direction(self.to_glib_none().0, direction.into_glib());
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_split_button_set_dropdown_tooltip")]
    pub fn set_dropdown_tooltip(&self, tooltip: &str) {
        unsafe {
            ffi::adw_split_button_set_dropdown_tooltip(
                self.to_glib_none().0,
                tooltip.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_split_button_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::adw_split_button_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_split_button_set_label")]
    pub fn set_label(&self, label: &str) {
        unsafe {
            ffi::adw_split_button_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_split_button_set_menu_model")]
    pub fn set_menu_model(&self, menu_model: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::adw_split_button_set_menu_model(
                self.to_glib_none().0,
                menu_model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_split_button_set_popover")]
    pub fn set_popover(&self, popover: Option<&impl IsA<gtk::Popover>>) {
        unsafe {
            ffi::adw_split_button_set_popover(
                self.to_glib_none().0,
                popover.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_split_button_set_use_underline")]
    pub fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::adw_split_button_set_use_underline(
                self.to_glib_none().0,
                use_underline.into_glib(),
            );
        }
    }

    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "clicked")]
    pub fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clicked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clicked_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_clicked(&self) {
        self.emit_by_name::<()>("clicked", &[]);
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "can-shrink")]
    pub fn connect_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_shrink_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::can-shrink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_shrink_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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

    #[doc(alias = "direction")]
    pub fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "dropdown-tooltip")]
    pub fn connect_dropdown_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dropdown_tooltip_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::dropdown-tooltip\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dropdown_tooltip_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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

    #[doc(alias = "label")]
    pub fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-model")]
    pub fn connect_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_model_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::menu-model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popover")]
    pub fn connect_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popover_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::popover\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popover_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-underline")]
    pub fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&SplitButton) + 'static>(
            this: *mut ffi::AdwSplitButton,
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
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SplitButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SplitButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SplitButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, SplitButton>,
}

impl SplitButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    pub fn can_shrink(self, can_shrink: bool) -> Self {
        Self {
            builder: self.builder.property("can-shrink", can_shrink),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn direction(self, direction: gtk::ArrowType) -> Self {
        Self {
            builder: self.builder.property("direction", direction),
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn dropdown_tooltip(self, dropdown_tooltip: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("dropdown-tooltip", dropdown_tooltip.into()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn menu_model(self, menu_model: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self
                .builder
                .property("menu-model", menu_model.clone().upcast()),
        }
    }

    pub fn popover(self, popover: &impl IsA<gtk::Popover>) -> Self {
        Self {
            builder: self.builder.property("popover", popover.clone().upcast()),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SplitButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SplitButton {
        self.builder.build()
    }
}

impl fmt::Display for SplitButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SplitButton")
    }
}
