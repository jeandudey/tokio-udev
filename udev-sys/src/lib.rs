#![cfg(target_os = "linux")]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_char, c_int, c_ulonglong, dev_t};

pub type udev = c_void;

#[link(name = "udev")]
extern "C" {
    pub fn udev_new() -> *mut udev;
    pub fn udev_ref(udev: *mut udev) -> *mut udev;
    pub fn udev_unref(udev: *mut udev) -> *mut udev;
}

pub type udev_monitor = c_void;

#[link(name = "udev")]
extern "C" {
    pub fn udev_monitor_new_from_netlink(
        udev: *mut udev,
        name: *const c_char,
    ) -> *mut udev_monitor;
    pub fn udev_monitor_ref(udev_monitor: *mut udev_monitor) -> *mut udev_monitor;
    pub fn udev_monitor_unref(udev_monitor: *mut udev_monitor) -> *mut udev_monitor;
}

#[link(name = "udev")]
extern "C" {
    pub fn udev_monitor_filter_update(udev_monitor: *mut udev_monitor) -> c_int;
    pub fn udev_monitor_filter_remove(udev_monitor: *mut udev_monitor) -> c_int;
    pub fn udev_monitor_filter_add_match_subsystem_devtype(
        udev_monitor: *mut udev_monitor,
        subsytem: *const c_char,
        devtype: *const c_char,
    ) -> c_int;
    pub fn udev_monitor_filter_add_match_tag(
        udev_monitor: *mut udev_monitor,
        tag: *const c_char
    ) -> c_int;
}

pub type udev_device = c_void;

#[link(name = "udev")]
extern "C" {
    pub fn udev_monitor_receive_device(udev_monitor: *mut udev_monitor) -> *mut udev_device;
    pub fn udev_monitor_enable_receiving(udev_monitor: *mut udev_monitor) -> c_int;
    pub fn udev_monitor_set_receive_buffer_size(
        udev_monitor: *mut udev_monitor,
        size: c_int,
    ) -> c_int;
    pub fn udev_monitor_get_fd(udev_monitor: *mut udev_monitor) -> c_int;
}

#[link(name = "udev")]
extern "C" {
    pub fn udev_device_new_from_syspath(
        udev: *mut udev,
        syspath: *const c_char,
    ) -> *mut udev_device;
    pub fn udev_device_new_from_devnum(
        udev: *mut udev,
        type_: c_char,
        devnum: dev_t,
    ) -> *mut udev_device;
    pub fn udev_device_new_from_subsystem_sysname(
        udev: *mut udev,
        subsystem: *const c_char,
        sysname: *const c_char,
    ) -> *mut udev_device;
    pub fn udev_device_new_from_device_id(
        udev: *mut udev,
        id: *const c_char,
    ) -> *mut udev_device;
    pub fn udev_device_new_from_environment(udev: *mut udev) -> *mut udev_device;
    pub fn udev_device_ref(udev_device: *mut udev_device) -> *mut udev_device;
    pub fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
}

pub type udev_list_entry = c_void;

#[link(name = "udev")]
extern "C" {
    pub fn udev_device_get_devpath(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_subsystem(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_devtype(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_syspath(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_sysname(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_sysnum(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_devnode(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_is_initialized(udev_device: *mut udev_device) -> c_int;
    pub fn udev_device_get_devlinks_list_entry(
        udev_device: *mut udev_device
    ) -> *mut udev_list_entry;
    pub fn udev_device_get_properties_list_entry(
        udev_device: *mut udev_device
    ) -> *mut udev_list_entry;
    pub fn udev_device_get_tags_list_entry(
        udev_device: *mut udev_device
    ) -> *mut udev_list_entry;
    pub fn udev_device_get_sysattr_list_entry(
        udev_device: *mut udev_device
    ) -> *mut udev_list_entry;
    pub fn udev_device_get_property_value(
        udev_device: *mut udev_device,
        key: *const c_char
    ) -> *const c_char;
    pub fn udev_device_get_driver(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_devnum(udev_device: *mut udev_device) -> dev_t;
    pub fn udev_device_get_action(udev_device: *mut udev_device) -> *const c_char;
    pub fn udev_device_get_seqnum(udev_device: *mut udev_device) -> c_ulonglong;
    pub fn udev_device_get_usec_since_initialized(
        udev_device: *mut udev_device
    ) -> c_ulonglong;
    pub fn udev_device_get_sysattr_value(
        udev_device: *mut udev_device,
        sysattr: *const c_char
    ) -> *const c_char;
    pub fn udev_device_set_sysattr_value(
        udev_device: *mut udev_device,
        sysattr: *const c_char,
        value: *mut c_char
    ) -> c_int;
    pub fn udev_device_has_tag(udev_device: *mut udev_device, tag: *const c_char) -> c_int;
}
