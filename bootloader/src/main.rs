#![no_main]
#![no_std]

use log::info;
use uefi::{
    prelude::*,
    table::boot::{MemoryType, OpenProtocolParams, OpenProtocolAttributes},
    proto::{
	media::fs::SimpleFileSystem,
	loaded_image::LoadedImage,
	device_path::DevicePath,
    },
};

const FILENAME: &'static str = "kernel-x86_64";

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // At this point uefi has done all the usual platform init, plus it has landed us
    // in long-mode with identity mapped pages.
    uefi_services::init(&mut system_table).unwrap();
    info!("memory::init()");
    info!("exit_boot_services()");
    let (mut _system_table, mut _memory_map) = system_table.exit_boot_services(MemoryType::CONVENTIONAL);
    Status::SUCCESS
}
