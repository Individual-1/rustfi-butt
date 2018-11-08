#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_exts;
extern crate uefi_services;

#[macro_use]
extern crate log;

use uefi::prelude::*;
use uefi::proto::media::file::{File};
use uefi::proto::media::fs::{SimpleFileSystem};
use uefi::table::boot::BootServices;
use uefi_exts::BootServicesExt;

#[no_mangle]
pub extern "win64" fn uefi_start(handle: uefi::Handle, st: SystemTable<Boot>) -> Status {

    // Init supplementary services
    uefi_services::init(&st).expect_success("Failed to initialize uefi_services");

    // Get boot services pointer
    let bt = st.boot_services();

    // Get runtime services pointer
    let rt = st.runtime_services();

    // Get Filesystem handle
    if let Some(sfs) = bt.find_protocol::<SimpleFileSystem>() {
        let sfs = unsafe { &mut *sfs.get() };

    } else {
        error!("Failed to load sfs handler");
    }

    /*
    // Get Filesystem handle
    if let Some(sfs) = bt.find_protocol::<SimpleFileSystem>() {
        let sfs = unsafe { &mut *sfs.get() };

    } else {
        error!("Failed to load sfs handler");
    }
    */
    // Get Filesystem handle
    let sfs = bt.find_protocol::<SimpleFileSystem>()
        .expect("Failed to load sfs handler");
    let sfs = unsafe { &mut *sfs.get() };

    let elf_file = match sfs.open_volume() {
        Ok(f) => f,
        Err(e) => error!("Failed to open volume: {}", e),
    }

    return;
}
