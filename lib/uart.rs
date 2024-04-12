use crate::demo_system::{enable_interrupts,set_global_interrupt_enable,UART_IRQ};

use core::ptr; 

#[repr(C)]
pub struct UartMmapRegs{
    uart_rx_reg : u32,
    uart_tx_reg : u32,
    uart_status_reg : u32,
}

const UART_STATUS_RX_EMPTY: u32 = 1;
const UART_STATUS_TX_FULL: u32 = 2;
const UART_EOF : i32 = -1;

pub type Uart = *mut UartMmapRegs;

pub fn uart_enable_rx_int(){
    
    unsafe {
        enable_interrupts(UART_IRQ);
        set_global_interrupt_enable(1);
    }

}

pub fn uart_in(uart : Uart) -> i32 {
    
    let mut res : i32 = UART_EOF;
    
    unsafe {
        if (ptr::read_volatile(&mut (*uart).uart_status_reg) & UART_STATUS_RX_EMPTY) == 0 {
            res = ptr::read_volatile(&mut (*uart).uart_rx_reg) as i32;
        }   
    }
    
    return res;
}

pub fn uart_out(uart : Uart, c : u8) {

    unsafe {
        
        while (ptr::read_volatile(&mut (*uart).uart_status_reg) & UART_STATUS_TX_FULL) != 0  {

        }

        ptr::write_volatile(&mut (*uart).uart_tx_reg, c as u32);

    }

}
