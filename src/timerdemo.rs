#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

use riscv_demosystem::peripherals;

global_asm!(include_str!("startup.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
extern "C" fn main() {

    let serial = unsafe {peripherals::PERIPHERALS.take_serial() };
    let timer = unsafe {peripherals::PERIPHERALS.take_timer()};

    let time_base : u64 = 4*1000;
    let current_time : u64;
    let mut elapsed_time : u64;

    serial.puts("TIMER DEMO\n");

    timer.timer_init();
    serial.puts("Timer initialized\n");

    timer.timer_enable(time_base);
    serial.puts("Timer enabled\n");

    current_time = timer.timer_read();
    elapsed_time = timer.get_elapsed_time();

    serial.puts("current time: ");
    serial.putdec(current_time as u32);
    serial.puts("\n");
    serial.puts("elapsed time: ");
    serial.putdec(elapsed_time as u32);
    serial.puts("\n\n\n");

    let mut start_time : u64;
    let mut end_time : u64;

    let mut i = 0;
    while i < 10 {

        start_time = timer.timer_read();

        let mut j = 0;
        while j < 10 {
            j = j + 1;
        }

        end_time = timer.timer_read();
        elapsed_time = end_time - start_time;

        serial.puts("elapsed time: ");
        serial.putdec(elapsed_time as u32);
        serial.puts("\t");
        serial.puts("start time: ");
        serial.putdec(start_time as u32);
        serial.puts("\t");
        serial.puts("end time: ");
        serial.putdec(end_time as u32);
        serial.puts("\n");

        i = i + 1;
    
    }


    timer.timer_disable();
    serial.puts("timer disabled\n");

}