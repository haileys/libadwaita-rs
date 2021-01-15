// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum CenteringPolicy {
    Loose,
    Strict,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for CenteringPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CenteringPolicy::{}",
            match *self {
                CenteringPolicy::Loose => "Loose",
                CenteringPolicy::Strict => "Strict",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for CenteringPolicy {
    type GlibType = ffi::HdyCenteringPolicy;

    fn to_glib(&self) -> ffi::HdyCenteringPolicy {
        match *self {
            CenteringPolicy::Loose => ffi::HDY_CENTERING_POLICY_LOOSE,
            CenteringPolicy::Strict => ffi::HDY_CENTERING_POLICY_STRICT,
            CenteringPolicy::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyCenteringPolicy> for CenteringPolicy {
    unsafe fn from_glib(value: ffi::HdyCenteringPolicy) -> Self {
        skip_assert_initialized!();
        match value {
            0 => CenteringPolicy::Loose,
            1 => CenteringPolicy::Strict,
            value => CenteringPolicy::__Unknown(value),
        }
    }
}

impl StaticType for CenteringPolicy {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_centering_policy_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CenteringPolicy {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CenteringPolicy {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CenteringPolicy {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FlapFoldPolicy {
    Never,
    Always,
    Auto,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FlapFoldPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FlapFoldPolicy::{}",
            match *self {
                FlapFoldPolicy::Never => "Never",
                FlapFoldPolicy::Always => "Always",
                FlapFoldPolicy::Auto => "Auto",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FlapFoldPolicy {
    type GlibType = ffi::HdyFlapFoldPolicy;

    fn to_glib(&self) -> ffi::HdyFlapFoldPolicy {
        match *self {
            FlapFoldPolicy::Never => ffi::HDY_FLAP_FOLD_POLICY_NEVER,
            FlapFoldPolicy::Always => ffi::HDY_FLAP_FOLD_POLICY_ALWAYS,
            FlapFoldPolicy::Auto => ffi::HDY_FLAP_FOLD_POLICY_AUTO,
            FlapFoldPolicy::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyFlapFoldPolicy> for FlapFoldPolicy {
    unsafe fn from_glib(value: ffi::HdyFlapFoldPolicy) -> Self {
        skip_assert_initialized!();
        match value {
            0 => FlapFoldPolicy::Never,
            1 => FlapFoldPolicy::Always,
            2 => FlapFoldPolicy::Auto,
            value => FlapFoldPolicy::__Unknown(value),
        }
    }
}

impl StaticType for FlapFoldPolicy {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_flap_fold_policy_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FlapFoldPolicy {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FlapFoldPolicy {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for FlapFoldPolicy {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FlapTransitionType {
    Over,
    Under,
    Slide,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FlapTransitionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FlapTransitionType::{}",
            match *self {
                FlapTransitionType::Over => "Over",
                FlapTransitionType::Under => "Under",
                FlapTransitionType::Slide => "Slide",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FlapTransitionType {
    type GlibType = ffi::HdyFlapTransitionType;

    fn to_glib(&self) -> ffi::HdyFlapTransitionType {
        match *self {
            FlapTransitionType::Over => ffi::HDY_FLAP_TRANSITION_TYPE_OVER,
            FlapTransitionType::Under => ffi::HDY_FLAP_TRANSITION_TYPE_UNDER,
            FlapTransitionType::Slide => ffi::HDY_FLAP_TRANSITION_TYPE_SLIDE,
            FlapTransitionType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyFlapTransitionType> for FlapTransitionType {
    unsafe fn from_glib(value: ffi::HdyFlapTransitionType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => FlapTransitionType::Over,
            1 => FlapTransitionType::Under,
            2 => FlapTransitionType::Slide,
            value => FlapTransitionType::__Unknown(value),
        }
    }
}

impl StaticType for FlapTransitionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_flap_transition_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FlapTransitionType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FlapTransitionType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for FlapTransitionType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum HeaderGroupChildType {
    HeaderBar,
    GtkHeaderBar,
    HeaderGroup,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for HeaderGroupChildType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HeaderGroupChildType::{}",
            match *self {
                HeaderGroupChildType::HeaderBar => "HeaderBar",
                HeaderGroupChildType::GtkHeaderBar => "GtkHeaderBar",
                HeaderGroupChildType::HeaderGroup => "HeaderGroup",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for HeaderGroupChildType {
    type GlibType = ffi::HdyHeaderGroupChildType;

    fn to_glib(&self) -> ffi::HdyHeaderGroupChildType {
        match *self {
            HeaderGroupChildType::HeaderBar => ffi::HDY_HEADER_GROUP_CHILD_TYPE_HEADER_BAR,
            HeaderGroupChildType::GtkHeaderBar => ffi::HDY_HEADER_GROUP_CHILD_TYPE_GTK_HEADER_BAR,
            HeaderGroupChildType::HeaderGroup => ffi::HDY_HEADER_GROUP_CHILD_TYPE_HEADER_GROUP,
            HeaderGroupChildType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyHeaderGroupChildType> for HeaderGroupChildType {
    unsafe fn from_glib(value: ffi::HdyHeaderGroupChildType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => HeaderGroupChildType::HeaderBar,
            1 => HeaderGroupChildType::GtkHeaderBar,
            2 => HeaderGroupChildType::HeaderGroup,
            value => HeaderGroupChildType::__Unknown(value),
        }
    }
}

impl StaticType for HeaderGroupChildType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_header_group_child_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for HeaderGroupChildType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for HeaderGroupChildType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for HeaderGroupChildType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum LeafletTransitionType {
    Over,
    Under,
    Slide,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for LeafletTransitionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LeafletTransitionType::{}",
            match *self {
                LeafletTransitionType::Over => "Over",
                LeafletTransitionType::Under => "Under",
                LeafletTransitionType::Slide => "Slide",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for LeafletTransitionType {
    type GlibType = ffi::HdyLeafletTransitionType;

    fn to_glib(&self) -> ffi::HdyLeafletTransitionType {
        match *self {
            LeafletTransitionType::Over => ffi::HDY_LEAFLET_TRANSITION_TYPE_OVER,
            LeafletTransitionType::Under => ffi::HDY_LEAFLET_TRANSITION_TYPE_UNDER,
            LeafletTransitionType::Slide => ffi::HDY_LEAFLET_TRANSITION_TYPE_SLIDE,
            LeafletTransitionType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyLeafletTransitionType> for LeafletTransitionType {
    unsafe fn from_glib(value: ffi::HdyLeafletTransitionType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => LeafletTransitionType::Over,
            1 => LeafletTransitionType::Under,
            2 => LeafletTransitionType::Slide,
            value => LeafletTransitionType::__Unknown(value),
        }
    }
}

impl StaticType for LeafletTransitionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_leaflet_transition_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for LeafletTransitionType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for LeafletTransitionType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for LeafletTransitionType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum NavigationDirection {
    Back,
    Forward,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for NavigationDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NavigationDirection::{}",
            match *self {
                NavigationDirection::Back => "Back",
                NavigationDirection::Forward => "Forward",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for NavigationDirection {
    type GlibType = ffi::HdyNavigationDirection;

    fn to_glib(&self) -> ffi::HdyNavigationDirection {
        match *self {
            NavigationDirection::Back => ffi::HDY_NAVIGATION_DIRECTION_BACK,
            NavigationDirection::Forward => ffi::HDY_NAVIGATION_DIRECTION_FORWARD,
            NavigationDirection::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyNavigationDirection> for NavigationDirection {
    unsafe fn from_glib(value: ffi::HdyNavigationDirection) -> Self {
        skip_assert_initialized!();
        match value {
            0 => NavigationDirection::Back,
            1 => NavigationDirection::Forward,
            value => NavigationDirection::__Unknown(value),
        }
    }
}

impl StaticType for NavigationDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_navigation_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for NavigationDirection {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for NavigationDirection {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for NavigationDirection {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SqueezerTransitionType {
    None,
    Crossfade,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SqueezerTransitionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SqueezerTransitionType::{}",
            match *self {
                SqueezerTransitionType::None => "None",
                SqueezerTransitionType::Crossfade => "Crossfade",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SqueezerTransitionType {
    type GlibType = ffi::HdySqueezerTransitionType;

    fn to_glib(&self) -> ffi::HdySqueezerTransitionType {
        match *self {
            SqueezerTransitionType::None => ffi::HDY_SQUEEZER_TRANSITION_TYPE_NONE,
            SqueezerTransitionType::Crossfade => ffi::HDY_SQUEEZER_TRANSITION_TYPE_CROSSFADE,
            SqueezerTransitionType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdySqueezerTransitionType> for SqueezerTransitionType {
    unsafe fn from_glib(value: ffi::HdySqueezerTransitionType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => SqueezerTransitionType::None,
            1 => SqueezerTransitionType::Crossfade,
            value => SqueezerTransitionType::__Unknown(value),
        }
    }
}

impl StaticType for SqueezerTransitionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_squeezer_transition_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SqueezerTransitionType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SqueezerTransitionType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SqueezerTransitionType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ViewSwitcherPolicy {
    Auto,
    Narrow,
    Wide,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ViewSwitcherPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ViewSwitcherPolicy::{}",
            match *self {
                ViewSwitcherPolicy::Auto => "Auto",
                ViewSwitcherPolicy::Narrow => "Narrow",
                ViewSwitcherPolicy::Wide => "Wide",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ViewSwitcherPolicy {
    type GlibType = ffi::HdyViewSwitcherPolicy;

    fn to_glib(&self) -> ffi::HdyViewSwitcherPolicy {
        match *self {
            ViewSwitcherPolicy::Auto => ffi::HDY_VIEW_SWITCHER_POLICY_AUTO,
            ViewSwitcherPolicy::Narrow => ffi::HDY_VIEW_SWITCHER_POLICY_NARROW,
            ViewSwitcherPolicy::Wide => ffi::HDY_VIEW_SWITCHER_POLICY_WIDE,
            ViewSwitcherPolicy::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyViewSwitcherPolicy> for ViewSwitcherPolicy {
    unsafe fn from_glib(value: ffi::HdyViewSwitcherPolicy) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ViewSwitcherPolicy::Auto,
            1 => ViewSwitcherPolicy::Narrow,
            2 => ViewSwitcherPolicy::Wide,
            value => ViewSwitcherPolicy::__Unknown(value),
        }
    }
}

impl StaticType for ViewSwitcherPolicy {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_view_switcher_policy_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ViewSwitcherPolicy {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ViewSwitcherPolicy {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ViewSwitcherPolicy {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
