// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    /// This method returns an instance of [`SplitButtonBuilder`] which can be used to create [`SplitButton`] objects.
    pub fn builder() -> SplitButtonBuilder {
        SplitButtonBuilder::default()
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

    #[doc(alias = "adw_split_button_set_child")]
    pub fn set_child<P: IsA<gtk::Widget>>(&self, child: Option<&P>) {
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
    pub fn set_menu_model<P: IsA<gio::MenuModel>>(&self, menu_model: Option<&P>) {
        unsafe {
            ffi::adw_split_button_set_menu_model(
                self.to_glib_none().0,
                menu_model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_split_button_set_popover")]
    pub fn set_popover<P: IsA<gtk::Popover>>(&self, popover: Option<&P>) {
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
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("activate", &[])
                .unwrap()
        };
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
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("clicked", &[])
                .unwrap()
        };
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

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SplitButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct SplitButtonBuilder {
    child: Option<gtk::Widget>,
    direction: Option<gtk::ArrowType>,
    icon_name: Option<String>,
    label: Option<String>,
    menu_model: Option<gio::MenuModel>,
    popover: Option<gtk::Popover>,
    use_underline: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<gtk::Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<gtk::LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<gtk::Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<gtk::Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<gtk::AccessibleRole>,
    action_name: Option<String>,
    action_target: Option<glib::Variant>,
}

impl SplitButtonBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SplitButtonBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SplitButton`].
    pub fn build(self) -> SplitButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref direction) = self.direction {
            properties.push(("direction", direction));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref menu_model) = self.menu_model {
            properties.push(("menu-model", menu_model));
        }
        if let Some(ref popover) = self.popover {
            properties.push(("popover", popover));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref action_target) = self.action_target {
            properties.push(("action-target", action_target));
        }
        glib::Object::new::<SplitButton>(&properties)
            .expect("Failed to create an instance of SplitButton")
    }

    pub fn child<P: IsA<gtk::Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn direction(mut self, direction: gtk::ArrowType) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn menu_model<P: IsA<gio::MenuModel>>(mut self, menu_model: &P) -> Self {
        self.menu_model = Some(menu_model.clone().upcast());
        self
    }

    pub fn popover<P: IsA<gtk::Popover>>(mut self, popover: &P) -> Self {
        self.popover = Some(popover.clone().upcast());
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<gtk::LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: gtk::Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: gtk::AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn action_target(mut self, action_target: &glib::Variant) -> Self {
        self.action_target = Some(action_target.clone());
        self
    }
}

impl fmt::Display for SplitButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SplitButton")
    }
}