#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

use drivers::demo_system;

global_asm!(include_str!("startup.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn main() {
    
    demo_system::puts("Hello World\n");

}
