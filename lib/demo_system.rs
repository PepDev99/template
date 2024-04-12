use core::ptr;

use core::arch::asm;

use crate::timer::TimerMmapRegs;

use self::uart::UartMmapRegs;

#[repr(C)]
struct SysMmapRegs{
    sim_ctrl_out: u32, /* ASCII Out, write ASCII characters here that will get output to the log file */
    __ : u32,
    sim_ctrl_ctrl: u32, /* Simulator Halt, write 1 here to halt the simulation */
}

const SIM_CTRL_BASE: *mut SysMmapRegs = 0x00020000 as *mut SysMmapRegs;

const UART0_BASE: *mut UartMmapRegs = 0x80001000 as *mut UartMmapRegs;

const GPIO_BASE: usize = 0x80000000;

pub const TIMER_BASE: *mut TimerMmapRegs = 0x80002000 as *mut TimerMmapRegs;

const UART_IRQ_NUM : u32 = 16;
const UART_IRQ : u32 = 1 << UART_IRQ_NUM;

pub const GPIO_OUT: *mut u32 = (GPIO_BASE + gpio::GPIO_OUT_REG) as *mut u32;
pub const GPIO_IN: *mut u32 = (GPIO_BASE + gpio::GPIO_IN_REG) as *mut u32;
pub const GPIO_IN_DBNC: *mut u32 = (GPIO_BASE + gpio::GPIO_IN_DBNC_REG) as *mut u32;
pub const GPIO_OUT_SHIFT: *mut u32 = (GPIO_BASE + gpio::GPIO_OUT_SHIFT_REG) as *mut u32;

pub const TIMER_IRQ : u32 = 1 << 7;

pub fn putchar(c : i32) -> i32 {

    #[cfg(feature = "sim_ctrl_output")] 
    {
        
        unsafe {
            ptr::write_volatile(&mut (*SIM_CTRL_BASE).sim_ctrl_out, c as u32);
        }

    }

    #[cfg(not(feature = "sim_ctrl_output"))] 
    {

        if (c as u8) == b'\n' {
            uart::uart_out(UART0_BASE as uart::Uart, b'\r');
        }

        uart::uart_out(UART0_BASE as uart::Uart, c as u8);
        
    }
    
    return c;

}

pub fn getchar() -> i32 {
    return uart::uart_in(UART0_BASE as uart::Uart);
}

pub unsafe fn sim_halt() {

    ptr::write_volatile(&mut (*SIM_CTRL_BASE).sim_ctrl_ctrl, 1 as u32);
    
}

pub fn puts(str: &str) -> i32 {

    for c in str.bytes() {
       putchar(c as i32);
    }

    return 0;
}

pub fn puthex(mut h: u32) {

    let mut cur_digit: i32;
    // Iterate through h taking top 4 bits each time and outputting ASCII of hex
    // digit for those 4 bits
    for _i in 0..8 {
        cur_digit = (h >> 28) as i32;
  
        if cur_digit < 10 {
            
            putchar((b'0' + (cur_digit as u8)) as i32);
        
        }
        else {
            
            putchar((b'A' - 10 + (cur_digit as u8)) as i32);

        }
  
      h <<= 4;
    
    }

}

pub fn putbyte(h: u32) {
    
    let mut cur_digit: i32;

    cur_digit = (h >> 4) as i32;
    
    if cur_digit < 10 {
        
        putchar((b'0' + (cur_digit as u8)) as i32);

    }
    else {

        putchar((b'A' - 10 + (cur_digit as u8)) as i32);

    }

    cur_digit = (h & 0x0f) as i32;
    
    if cur_digit < 10 {
        
        putchar((b'0' + (cur_digit as u8)) as i32);

    }
    else {

        putchar((b'A' - 10 + (cur_digit as u8)) as i32);

    }

}

pub fn putdec(n: u32) {

    if n > 9 {
        
        putdec(n/10);

    }
	
    putchar((b'0' + ((n % 10) as u8)) as i32);

}

extern "C" {
    static _vectors_start: u32;
    
}

pub unsafe fn install_exception_handler(vector_num : u32, handler_fn : extern "riscv-interrupt-m" fn()) -> u32 {

    let exc_vectors: *const u32 = &_vectors_start;

    if vector_num >= 32 {
        return 1;
    }
    
    let handler_jmp_loc = exc_vectors.add(vector_num as usize);
    
    let offset : i32 = (handler_fn as u32).wrapping_sub(handler_jmp_loc as u32) as i32;
    
    if (offset  >= (1 << 19)) || (offset  < -(1 << 19)) {
        return 2;
    }

    let offset_uimm : u32 = offset as u32;

    let jmp_ins : u32 = 
        ((offset_uimm & 0x7fe) << 20) | // imm[10:1] -> 21
        ((offset_uimm & 0x800) << 9) | // imm[11] -> 20
        (offset_uimm & 0xff000) | // imm[19:12] -> 12
        ((offset_uimm & 0x100000) << 11) | // imm[20] -> 31
        (0x6f); // J opcode

    ptr::write_volatile(handler_jmp_loc as *mut u32, jmp_ins);

    asm!("fence.i");
            
    return 0;

}


pub unsafe fn enable_interrupts(enable_mask : u32) {
    
    asm!("csrs mie, {}", in(reg) enable_mask);

}


pub unsafe fn disable_interrupts(disable_mask : u32) {
    
    asm!("csrc mie, {}", in(reg) disable_mask);
    
}

pub unsafe fn set_global_interrupt_enable(enable : u32) {
    if enable != 0 {
        asm!("csrs mstatus, {}", in(reg) (1 << 3));
    }

    else {
        asm!("csrc mstatus, {}", in(reg) (1 << 3));
    }

}

pub unsafe fn get_mepc() -> u32 {
    
    let result: u32;
    
    asm!("csrr {}, mepc", out(reg) result);

    return result;

}

pub unsafe fn get_mcause() -> u32 {
    
    let result: u32;
    
    asm!("csrr {}, mcause", out(reg) result);

    return result;

}

pub unsafe fn get_mtval() -> u32 {

    let result: u32;
    
    asm!("csrr {}, mtval", out(reg) result);

    return result;

}

pub unsafe fn get_mcycle() -> u32 {

    let result: u32;
    
    asm!("csrr {}, mcycle", out(reg) result);

    return result;


}

pub unsafe fn reset_mcycle() {

    asm!("csrw mcycle, x0");

}

#[path = "./uart.rs"] pub mod uart;

#[path = "./gpio.rs"] pub mod gpio;