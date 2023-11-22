#![no_std]

#[no_mangle]
pub extern "C" fn kmain() {
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
