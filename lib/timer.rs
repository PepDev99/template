use core::ptr;

use core::arch::asm;

use crate::demo_system::{enable_interrupts, install_exception_handler, set_global_interrupt_enable, TIMER_BASE, TIMER_IRQ};

#[repr(C)]
pub struct TimerMmapRegs{
    timer_mtime_reg : u32,
    timer_mtimeh_reg : u32,
    timer_mtimecmp_reg : u32,
    timer_mtimecmph_reg : u32,
}

static mut time_elapsed: u64 = 0;

static mut time_increment : u64 = 0;

unsafe fn timecmp_update(new_time : u64) {

    ptr::write_volatile(&mut (*TIMER_BASE).timer_mtimecmp_reg, u32::MAX);
    ptr::write_volatile(&mut (*TIMER_BASE).timer_mtimecmph_reg, (new_time >> 32) as u32);
    ptr::write_volatile(&mut (*TIMER_BASE).timer_mtimecmp_reg, new_time as u32);

}

#[inline]
unsafe fn increment_timecmp(time_base : u64) {

    let mut current_time : u64 = timer_read();

    current_time += time_base;

    timecmp_update(current_time);

}


extern "riscv-interrupt-m" fn simple_timer_handler() {

    unsafe {

        increment_timecmp(time_increment);
        
        ptr::write_volatile(&mut time_elapsed, time_elapsed + 1);
        
    }

}


pub fn timer_init() {
    
    unsafe {
        install_exception_handler(7, simple_timer_handler);
    }

}

pub fn timer_read() -> u64 {

    let mut current_timeh : u32;
    let mut current_time : u32;

    unsafe {
        loop {
        
            current_timeh = ptr::read_volatile(&mut (*TIMER_BASE).timer_mtimeh_reg);
            current_time = ptr::read_volatile(&mut (*TIMER_BASE).timer_mtime_reg);

            if current_timeh == ptr::read_volatile(&mut (*TIMER_BASE).timer_mtimeh_reg) {break;}
        
        }

    }

    let final_time : u64 = ((current_timeh as u64) << 32) | current_time as u64;

    return final_time;

}

pub fn get_elapsed_time() -> u64 {
    
    unsafe {
        return ptr::read_volatile(&mut time_elapsed);
    }

}

pub fn timer_enable(time_base : u64) {

    unsafe {
        ptr::write_volatile(&mut time_elapsed, 0);
        time_increment = time_base;
        increment_timecmp(time_base);
        enable_interrupts(TIMER_IRQ);
        set_global_interrupt_enable(1);
    }

}

pub fn timer_disable() {

    unsafe {
        asm!("csrc mie, {}", in(reg) (0x80));
    }
}


