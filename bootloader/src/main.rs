#![no_main]
#![no_std]

mod memory;

use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // At this point uefi has done all the usual platform init, plus it has landed us
    // in long-mode with identity mapped pages.
    uefi_services::init(&mut system_table).unwrap();
    info!("memory::init()");
    memory::init();
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
