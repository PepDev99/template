use core::ptr;

use core::arch::asm;

use crate::interrupt::{enable_interrupts, install_exception_handler, set_global_interrupt_enable};

pub (crate) const TIMER_BASE: *mut u32 = 0x80002000 as *mut u32;

const TIMER_IRQ : u32 = 1 << 7;

pub struct Timer {
    pub (crate) p: *mut u32
}

enum OffsetTimerReg {
    MTimeReg = 0,
    MTimeHReg = 1,
    MTimeCmpReg = 2,
    MTimeCmpHReg = 3,
}

static mut time_elapsed: u64 = 0;

static mut time_increment : u64 = 0;

impl Timer {

    #[inline]
    fn write_timer(&self, offset : OffsetTimerReg, value: u32 ) {
        
        unsafe {

            ptr::write_volatile(&mut *(self.p.add(offset as usize)), value);

        }

    }

    #[inline]
    fn read_timer(&self, offset : OffsetTimerReg) -> u32 {

        unsafe {
            
            ptr::read_volatile(&mut *(self.p.add(offset as usize)))

        }

    }

    fn timecmp_update(&self, new_time : u64) {

        self.write_timer(OffsetTimerReg::MTimeCmpReg, u32::MAX);
        self.write_timer(OffsetTimerReg::MTimeCmpHReg, (new_time >> 32) as u32);
        self.write_timer(OffsetTimerReg::MTimeCmpReg, new_time as u32 );
        
    }

    #[inline]
    fn increment_timecmp(&self, time_base : u64) {

        let mut current_time : u64 = self.timer_read();

        current_time += time_base;

        self.timecmp_update(current_time);

    }

    pub fn timer_init(&self) {
    
        unsafe {
            install_exception_handler(7, simple_timer_handler);
        }

    }   

    pub fn timer_read(&self) -> u64 {

        let mut current_timeh : u32;
        let mut current_time : u32;

        loop {
        
            current_timeh = self.read_timer(OffsetTimerReg::MTimeHReg);
            current_time = self.read_timer(OffsetTimerReg::MTimeReg);

            if current_timeh == self.read_timer(OffsetTimerReg::MTimeHReg) {break;}
        
        }

        let final_time : u64 = ((current_timeh as u64) << 32) | current_time as u64;

        final_time
    
    }

    pub fn get_elapsed_time(&self) -> u64 {
    
        unsafe {
            return ptr::read_volatile(&mut time_elapsed);
        }

    }   

    pub fn timer_enable(&self, time_base : u64) {

        unsafe {
            
            ptr::write_volatile(&mut time_elapsed, 0);
            time_increment = time_base;
            self.increment_timecmp(time_base);
            enable_interrupts(TIMER_IRQ);
            set_global_interrupt_enable(1);
            
        }

    }

    pub fn timer_disable(&self) {

        unsafe {
            asm!("csrc mie, {}", in(reg) (0x80));
        }
    }   

}

extern "riscv-interrupt-m" fn simple_timer_handler() {

    unsafe {
        
        let timer = Timer{p: TIMER_BASE};

        timer.increment_timecmp(time_increment);
    
        ptr::write_volatile(&mut time_elapsed, time_elapsed + 1);
    
    }

}