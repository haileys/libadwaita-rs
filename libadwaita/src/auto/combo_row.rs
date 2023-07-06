// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ActionRow, PreferencesRow};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AdwComboRow")]
    pub struct ComboRow(Object<ffi::AdwComboRow, ffi::AdwComboRowClass>) @extends ActionRow, PreferencesRow, gtk::ListBoxRow, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::adw_combo_row_get_type(),
    }
}

impl ComboRow {
    pub const NONE: Option<&'static ComboRow> = None;

    #[doc(alias = "adw_combo_row_new")]
    pub fn new() -> ComboRow {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_combo_row_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ComboRow`] objects.
    ///
    /// This method returns an instance of [`ComboRowBuilder`](crate::builders::ComboRowBuilder) which can be used to create [`ComboRow`] objects.
    pub fn builder() -> ComboRowBuilder {
        ComboRowBuilder::new()
    }
}

impl Default for ComboRow {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ComboRow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ComboRowBuilder {
    builder: glib::object::ObjectBuilder<'static, ComboRow>,
}

impl ComboRowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn expression(self, expression: impl AsRef<gtk::Expression>) -> Self {
        Self {
            builder: self
                .builder
                .property("expression", expression.as_ref().clone()),
        }
    }

    pub fn factory(self, factory: &impl IsA<gtk::ListItemFactory>) -> Self {
        Self {
            builder: self.builder.property("factory", factory.clone().upcast()),
        }
    }

    pub fn list_factory(self, list_factory: &impl IsA<gtk::ListItemFactory>) -> Self {
        Self {
            builder: self
                .builder
                .property("list-factory", list_factory.clone().upcast()),
        }
    }

    pub fn model(self, model: &impl IsA<gio::ListModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn selected(self, selected: u32) -> Self {
        Self {
            builder: self.builder.property("selected", selected),
        }
    }

    pub fn use_subtitle(self, use_subtitle: bool) -> Self {
        Self {
            builder: self.builder.property("use-subtitle", use_subtitle),
        }
    }

    pub fn activatable_widget(self, activatable_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("activatable-widget", activatable_widget.clone().upcast()),
        }
    }

    #[cfg_attr(feature = "v1_3", deprecated = "Since 1.3")]
    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn subtitle(self, subtitle: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("subtitle", subtitle.into()),
        }
    }

    pub fn subtitle_lines(self, subtitle_lines: i32) -> Self {
        Self {
            builder: self.builder.property("subtitle-lines", subtitle_lines),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn subtitle_selectable(self, subtitle_selectable: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("subtitle-selectable", subtitle_selectable),
        }
    }

    pub fn title_lines(self, title_lines: i32) -> Self {
        Self {
            builder: self.builder.property("title-lines", title_lines),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "v1_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_1")))]
    pub fn title_selectable(self, title_selectable: bool) -> Self {
        Self {
            builder: self.builder.property("title-selectable", title_selectable),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub fn use_markup(self, use_markup: bool) -> Self {
        Self {
            builder: self.builder.property("use-markup", use_markup),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn activatable(self, activatable: bool) -> Self {
        Self {
            builder: self.builder.property("activatable", activatable),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            builder: self.builder.property("selectable", selectable),
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
    /// Build the [`ComboRow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ComboRow {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ComboRow>> Sealed for T {}
}

pub trait ComboRowExt: IsA<ComboRow> + sealed::Sealed + 'static {
    #[doc(alias = "adw_combo_row_get_expression")]
    #[doc(alias = "get_expression")]
    fn expression(&self) -> Option<gtk::Expression> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_expression(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_combo_row_get_factory")]
    #[doc(alias = "get_factory")]
    fn factory(&self) -> Option<gtk::ListItemFactory> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_combo_row_get_list_factory")]
    #[doc(alias = "get_list_factory")]
    fn list_factory(&self) -> Option<gtk::ListItemFactory> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_list_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_combo_row_get_model")]
    #[doc(alias = "get_model")]
    fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::adw_combo_row_get_model(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "adw_combo_row_get_selected")]
    #[doc(alias = "get_selected")]
    fn selected(&self) -> u32 {
        unsafe { ffi::adw_combo_row_get_selected(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "adw_combo_row_get_selected_item")]
    #[doc(alias = "get_selected_item")]
    fn selected_item(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_selected_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_combo_row_get_use_subtitle")]
    #[doc(alias = "get_use_subtitle")]
    fn uses_subtitle(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_combo_row_get_use_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_combo_row_set_expression")]
    fn set_expression(&self, expression: Option<impl AsRef<gtk::Expression>>) {
        unsafe {
            ffi::adw_combo_row_set_expression(
                self.as_ref().to_glib_none().0,
                expression.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_combo_row_set_factory")]
    fn set_factory(&self, factory: Option<&impl IsA<gtk::ListItemFactory>>) {
        unsafe {
            ffi::adw_combo_row_set_factory(
                self.as_ref().to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_combo_row_set_list_factory")]
    fn set_list_factory(&self, factory: Option<&impl IsA<gtk::ListItemFactory>>) {
        unsafe {
            ffi::adw_combo_row_set_list_factory(
                self.as_ref().to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_combo_row_set_model")]
    fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::adw_combo_row_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_combo_row_set_selected")]
    fn set_selected(&self, position: u32) {
        unsafe {
            ffi::adw_combo_row_set_selected(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "adw_combo_row_set_use_subtitle")]
    fn set_use_subtitle(&self, use_subtitle: bool) {
        unsafe {
            ffi::adw_combo_row_set_use_subtitle(
                self.as_ref().to_glib_none().0,
                use_subtitle.into_glib(),
            );
        }
    }

    #[doc(alias = "expression")]
    fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<P: IsA<ComboRow>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "factory")]
    fn connect_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<P: IsA<ComboRow>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factory_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "list-factory")]
    fn connect_list_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_list_factory_trampoline<
            P: IsA<ComboRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::list-factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_list_factory_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P: IsA<ComboRow>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected")]
    fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<P: IsA<ComboRow>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-item")]
    fn connect_selected_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_item_trampoline<
            P: IsA<ComboRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_item_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-subtitle")]
    fn connect_use_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_subtitle_trampoline<
            P: IsA<ComboRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ComboRow>> ComboRowExt for O {}

impl fmt::Display for ComboRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ComboRow")
    }
}
