// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]

extern crate gdk_pixbuf_sys as gdk_pixbuf;
extern crate gdk_sys as gdk;
extern crate gio_sys as gio;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate libc;
extern crate pango_sys as pango;

mod manual;

pub use manual::*;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Records
#[repr(C)]
pub struct _GdkX11AppLaunchContextClass(c_void);

pub type GdkX11AppLaunchContextClass = *mut _GdkX11AppLaunchContextClass;

#[repr(C)]
pub struct _GdkX11CursorClass(c_void);

pub type GdkX11CursorClass = *mut _GdkX11CursorClass;

#[repr(C)]
pub struct _GdkX11DeviceCoreClass(c_void);

pub type GdkX11DeviceCoreClass = *mut _GdkX11DeviceCoreClass;

#[repr(C)]
pub struct _GdkX11DeviceManagerCoreClass(c_void);

pub type GdkX11DeviceManagerCoreClass = *mut _GdkX11DeviceManagerCoreClass;

#[repr(C)]
pub struct _GdkX11DeviceManagerXI2Class(c_void);

pub type GdkX11DeviceManagerXI2Class = *mut _GdkX11DeviceManagerXI2Class;

#[repr(C)]
pub struct _GdkX11DeviceXI2Class(c_void);

pub type GdkX11DeviceXI2Class = *mut _GdkX11DeviceXI2Class;

#[repr(C)]
pub struct _GdkX11DisplayClass(c_void);

pub type GdkX11DisplayClass = *mut _GdkX11DisplayClass;

#[repr(C)]
pub struct _GdkX11DisplayManagerClass(c_void);

pub type GdkX11DisplayManagerClass = *mut _GdkX11DisplayManagerClass;

#[repr(C)]
pub struct _GdkX11DragContextClass(c_void);

pub type GdkX11DragContextClass = *mut _GdkX11DragContextClass;

#[repr(C)]
pub struct _GdkX11GLContextClass(c_void);

pub type GdkX11GLContextClass = *mut _GdkX11GLContextClass;

#[repr(C)]
pub struct _GdkX11KeymapClass(c_void);

pub type GdkX11KeymapClass = *mut _GdkX11KeymapClass;

#[repr(C)]
pub struct _GdkX11MonitorClass(c_void);

pub type GdkX11MonitorClass = *mut _GdkX11MonitorClass;

#[repr(C)]
pub struct _GdkX11ScreenClass(c_void);

pub type GdkX11ScreenClass = *mut _GdkX11ScreenClass;

#[repr(C)]
pub struct _GdkX11VisualClass(c_void);

pub type GdkX11VisualClass = *mut _GdkX11VisualClass;

#[repr(C)]
pub struct _GdkX11WindowClass(c_void);

pub type GdkX11WindowClass = *mut _GdkX11WindowClass;

// Classes
#[repr(C)]
pub struct GdkX11AppLaunchContext(c_void);

impl ::std::fmt::Debug for GdkX11AppLaunchContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11AppLaunchContext @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Cursor(c_void);

impl ::std::fmt::Debug for GdkX11Cursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Cursor @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DeviceCore(c_void);

impl ::std::fmt::Debug for GdkX11DeviceCore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DeviceCore @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DeviceManagerCore(c_void);

impl ::std::fmt::Debug for GdkX11DeviceManagerCore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DeviceManagerCore @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DeviceManagerXI2(c_void);

impl ::std::fmt::Debug for GdkX11DeviceManagerXI2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DeviceManagerXI2 @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DeviceXI2(c_void);

impl ::std::fmt::Debug for GdkX11DeviceXI2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DeviceXI2 @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Display(c_void);

impl ::std::fmt::Debug for GdkX11Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Display @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DisplayManager(c_void);

impl ::std::fmt::Debug for GdkX11DisplayManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DisplayManager @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DragContext(c_void);

impl ::std::fmt::Debug for GdkX11DragContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DragContext @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11GLContext(c_void);

impl ::std::fmt::Debug for GdkX11GLContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11GLContext @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Keymap(c_void);

impl ::std::fmt::Debug for GdkX11Keymap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Keymap @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Monitor(c_void);

impl ::std::fmt::Debug for GdkX11Monitor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Monitor @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Screen(c_void);

impl ::std::fmt::Debug for GdkX11Screen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Screen @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Visual(c_void);

impl ::std::fmt::Debug for GdkX11Visual {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Visual @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Window(c_void);

impl ::std::fmt::Debug for GdkX11Window {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Window @ {:?}", self as *const _))
            .finish()
    }
}

extern "C" {

    //=========================================================================
    // GdkX11AppLaunchContext
    //=========================================================================
    pub fn gdk_x11_app_launch_context_get_type() -> GType;

    //=========================================================================
    // GdkX11Cursor
    //=========================================================================
    pub fn gdk_x11_cursor_get_type() -> GType;
    pub fn gdk_x11_cursor_get_xcursor(cursor: *mut GdkX11Cursor) -> xlib::Cursor;
    pub fn gdk_x11_cursor_get_xdisplay(cursor: *mut GdkX11Cursor) -> *mut xlib::Display;

    //=========================================================================
    // GdkX11DeviceCore
    //=========================================================================
    pub fn gdk_x11_device_core_get_type() -> GType;

    //=========================================================================
    // GdkX11DeviceManagerCore
    //=========================================================================
    pub fn gdk_x11_device_manager_core_get_type() -> GType;

    //=========================================================================
    // GdkX11DeviceManagerXI2
    //=========================================================================
    pub fn gdk_x11_device_manager_xi2_get_type() -> GType;

    //=========================================================================
    // GdkX11DeviceXI2
    //=========================================================================
    pub fn gdk_x11_device_xi2_get_type() -> GType;

    //=========================================================================
    // GdkX11Display
    //=========================================================================
    pub fn gdk_x11_display_get_type() -> GType;
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn gdk_x11_display_get_glx_version(
        display: *mut gdk::GdkDisplay,
        major: *mut c_int,
        minor: *mut c_int,
    ) -> gboolean;
    pub fn gdk_x11_display_broadcast_startup_message(
        display: *mut GdkX11Display,
        message_type: *const c_char,
        ...
    );
    pub fn gdk_x11_display_error_trap_pop(display: *mut GdkX11Display) -> c_int;
    pub fn gdk_x11_display_error_trap_pop_ignored(display: *mut GdkX11Display);
    pub fn gdk_x11_display_error_trap_push(display: *mut GdkX11Display);
    pub fn gdk_x11_display_get_startup_notification_id(
        display: *mut GdkX11Display,
    ) -> *const c_char;
    pub fn gdk_x11_display_get_user_time(display: *mut GdkX11Display) -> u32;
    pub fn gdk_x11_display_get_xdisplay(display: *mut GdkX11Display) -> *mut xlib::Display;
    pub fn gdk_x11_display_grab(display: *mut GdkX11Display);
    pub fn gdk_x11_display_set_cursor_theme(
        display: *mut GdkX11Display,
        theme: *const c_char,
        size: c_int,
    );
    pub fn gdk_x11_display_set_startup_notification_id(
        display: *mut GdkX11Display,
        startup_id: *const c_char,
    );
    pub fn gdk_x11_display_set_window_scale(display: *mut GdkX11Display, scale: c_int);
    pub fn gdk_x11_display_string_to_compound_text(
        display: *mut GdkX11Display,
        str: *const c_char,
        encoding: *mut gdk::GdkAtom,
        format: *mut c_int,
        ctext: *mut *mut u8,
        length: *mut c_int,
    ) -> c_int;
    pub fn gdk_x11_display_text_property_to_text_list(
        display: *mut GdkX11Display,
        encoding: gdk::GdkAtom,
        format: c_int,
        text: *const u8,
        length: c_int,
        list: *mut *mut *mut c_char,
    ) -> c_int;
    pub fn gdk_x11_display_ungrab(display: *mut GdkX11Display);
    pub fn gdk_x11_display_utf8_to_compound_text(
        display: *mut GdkX11Display,
        str: *const c_char,
        encoding: *mut gdk::GdkAtom,
        format: *mut c_int,
        ctext: *mut *mut u8,
        length: *mut c_int,
    ) -> gboolean;

    //=========================================================================
    // GdkX11DisplayManager
    //=========================================================================
    pub fn gdk_x11_display_manager_get_type() -> GType;

    //=========================================================================
    // GdkX11DragContext
    //=========================================================================
    pub fn gdk_x11_drag_context_get_type() -> GType;

    //=========================================================================
    // GdkX11GLContext
    //=========================================================================
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn gdk_x11_gl_context_get_type() -> GType;

    //=========================================================================
    // GdkX11Keymap
    //=========================================================================
    pub fn gdk_x11_keymap_get_type() -> GType;
    pub fn gdk_x11_keymap_get_group_for_state(keymap: *mut GdkX11Keymap, state: c_uint) -> c_int;
    pub fn gdk_x11_keymap_key_is_modifier(keymap: *mut GdkX11Keymap, keycode: c_uint) -> gboolean;

    //=========================================================================
    // GdkX11Monitor
    //=========================================================================
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn gdk_x11_monitor_get_type() -> GType;
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn gdk_x11_monitor_get_output(monitor: *mut gdk::GdkMonitor) -> xlib::XID;

    //=========================================================================
    // GdkX11Screen
    //=========================================================================
    pub fn gdk_x11_screen_get_type() -> GType;
    pub fn gdk_x11_screen_get_current_desktop(screen: *mut GdkX11Screen) -> u32;
    pub fn gdk_x11_screen_get_monitor_output(
        screen: *mut GdkX11Screen,
        monitor_num: c_int,
    ) -> xlib::XID;
    pub fn gdk_x11_screen_get_number_of_desktops(screen: *mut GdkX11Screen) -> u32;
    pub fn gdk_x11_screen_get_screen_number(screen: *mut GdkX11Screen) -> c_int;
    pub fn gdk_x11_screen_get_window_manager_name(screen: *mut GdkX11Screen) -> *const c_char;
    pub fn gdk_x11_screen_get_xscreen(screen: *mut GdkX11Screen) -> *mut xlib::Screen;
    pub fn gdk_x11_screen_lookup_visual(
        screen: *mut GdkX11Screen,
        xvisualid: xlib::VisualID,
    ) -> *mut GdkX11Visual;
    pub fn gdk_x11_screen_supports_net_wm_hint(
        screen: *mut GdkX11Screen,
        property: gdk::GdkAtom,
    ) -> gboolean;

    //=========================================================================
    // GdkX11Visual
    //=========================================================================
    pub fn gdk_x11_visual_get_type() -> GType;
    pub fn gdk_x11_visual_get_xvisual(visual: *mut GdkX11Visual) -> *mut xlib::Visual;

    //=========================================================================
    // GdkX11Window
    //=========================================================================
    pub fn gdk_x11_window_get_type() -> GType;
    pub fn gdk_x11_window_foreign_new_for_display(
        display: *mut GdkX11Display,
        window: xlib::Window,
    ) -> *mut gdk::GdkWindow;
    pub fn gdk_x11_window_lookup_for_display(
        display: *mut GdkX11Display,
        window: xlib::Window,
    ) -> *mut GdkX11Window;
    pub fn gdk_x11_window_get_desktop(window: *mut GdkX11Window) -> u32;
    pub fn gdk_x11_window_get_xid(window: *mut GdkX11Window) -> xlib::Window;
    pub fn gdk_x11_window_move_to_current_desktop(window: *mut GdkX11Window);
    pub fn gdk_x11_window_move_to_desktop(window: *mut GdkX11Window, desktop: u32);
    pub fn gdk_x11_window_set_frame_extents(
        window: *mut GdkX11Window,
        left: c_int,
        right: c_int,
        top: c_int,
        bottom: c_int,
    );
    pub fn gdk_x11_window_set_frame_sync_enabled(
        window: *mut GdkX11Window,
        frame_sync_enabled: gboolean,
    );
    pub fn gdk_x11_window_set_hide_titlebar_when_maximized(
        window: *mut GdkX11Window,
        hide_titlebar_when_maximized: gboolean,
    );
    pub fn gdk_x11_window_set_theme_variant(window: *mut GdkX11Window, variant: *mut c_char);
    pub fn gdk_x11_window_set_user_time(window: *mut GdkX11Window, timestamp: u32);
    pub fn gdk_x11_window_set_utf8_property(
        window: *mut GdkX11Window,
        name: *const c_char,
        value: *const c_char,
    );

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gdk_x11_atom_to_xatom(atom: gdk::GdkAtom) -> xlib::Atom;
    pub fn gdk_x11_atom_to_xatom_for_display(
        display: *mut GdkX11Display,
        atom: gdk::GdkAtom,
    ) -> xlib::Atom;
    pub fn gdk_x11_device_get_id(device: *mut GdkX11DeviceCore) -> c_int;
    pub fn gdk_x11_device_manager_lookup(
        device_manager: *mut GdkX11DeviceManagerCore,
        device_id: c_int,
    ) -> *mut GdkX11DeviceCore;
    pub fn gdk_x11_free_compound_text(ctext: *mut u8);
    pub fn gdk_x11_free_text_list(list: *mut *mut c_char);
    pub fn gdk_x11_get_default_root_xwindow() -> xlib::Window;
    pub fn gdk_x11_get_default_screen() -> c_int;
    pub fn gdk_x11_get_default_xdisplay() -> *mut xlib::Display;
    #[cfg(any(feature = "v3_24_2", feature = "dox"))]
    pub fn gdk_x11_get_parent_relative_pattern() -> *mut cairo::cairo_pattern_t;
    pub fn gdk_x11_get_server_time(window: *mut GdkX11Window) -> u32;
    pub fn gdk_x11_get_xatom_by_name(atom_name: *const c_char) -> xlib::Atom;
    pub fn gdk_x11_get_xatom_by_name_for_display(
        display: *mut GdkX11Display,
        atom_name: *const c_char,
    ) -> xlib::Atom;
    pub fn gdk_x11_get_xatom_name(xatom: xlib::Atom) -> *const c_char;
    pub fn gdk_x11_get_xatom_name_for_display(
        display: *mut GdkX11Display,
        xatom: xlib::Atom,
    ) -> *const c_char;
    pub fn gdk_x11_grab_server();
    pub fn gdk_x11_lookup_xdisplay(xdisplay: *mut xlib::Display) -> *mut GdkX11Display;
    pub fn gdk_x11_register_standard_event_type(
        display: *mut GdkX11Display,
        event_base: c_int,
        n_events: c_int,
    );
    pub fn gdk_x11_set_sm_client_id(sm_client_id: *const c_char);
    pub fn gdk_x11_ungrab_server();
    pub fn gdk_x11_xatom_to_atom(xatom: xlib::Atom) -> gdk::GdkAtom;
    pub fn gdk_x11_xatom_to_atom_for_display(
        display: *mut GdkX11Display,
        xatom: xlib::Atom,
    ) -> gdk::GdkAtom;

}
