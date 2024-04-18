#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

use drivers::peripherals;

global_asm!(include_str!("startup.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn main() {

    let serial = unsafe {peripherals::PERIPHERALS.take_serial() };
    let gpio = unsafe { peripherals::PERIPHERALS.take_gpio() };

    serial.puts("GPIO Test\n");

    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut s = 0;


    for element in v {
        s += element;
    }

    s = 10;

    gpio.set_output(s);
    
    

    gpio.set_output(10);
    
}