use crate::interrupt::{enable_interrupts,set_global_interrupt_enable};

use core::ptr;

pub (crate) const UART0_BASE: *mut u32 = 0x80001000 as *mut u32;

const UART_IRQ_NUM : u32 = 16;

const UART_IRQ : u32 = 1 << UART_IRQ_NUM;

const UART_STATUS_RX_EMPTY: u32 = 1;
const UART_STATUS_TX_FULL: u32 = 2;
const UART_EOF : i32 = -1;

pub struct Uart {
    pub (crate) p: *mut u32
}

enum OffsetUartReg {
    RxReg = 0,
    TxReg = 1,
    StatusReg = 2,
}


impl Uart {

    #[no_mangle]
    #[inline(always)]
    fn write_uart(&self, offset : OffsetUartReg, value: u32 ) {
        
        unsafe {

            ptr::write_volatile(&mut *(self.p.add(offset as usize)), value);
        
        }

    }

    #[no_mangle]
    #[inline(always)]
    fn read_uart(&self, offset : OffsetUartReg) -> u32 {

        unsafe {
            
            ptr::read_volatile(&mut *(self.p.add(offset as usize)))
        
        }

    }

    #[no_mangle]
    pub fn uart_enable_rx_int(){
    
        unsafe {
            enable_interrupts(UART_IRQ);
            set_global_interrupt_enable(1);
        }
    
    }

    #[no_mangle]
    pub fn uart_in(&self) -> i32 {
    
        let mut res : i32 = UART_EOF;
        
        if (self.read_uart(OffsetUartReg::StatusReg) & UART_STATUS_RX_EMPTY) == 0 {
            res = self.read_uart(OffsetUartReg::RxReg) as i32;
        }   
        
        res

    }

    #[no_mangle]
    pub fn uart_out(&self, c : u8) {

        while (self.read_uart(OffsetUartReg::StatusReg) & UART_STATUS_TX_FULL) != 0  {
    
        }
    
        self.write_uart(OffsetUartReg::TxReg, c as u32);
    
    }

    #[no_mangle]
    pub fn putchar(&self, c : i32) -> i32 {
    
        if (c as u8) == b'\n' {
            
            self.uart_out(b'\r');
    
        }
    
        self.uart_out(c as u8);
            
        c
    
    }

    #[no_mangle]
    pub fn getchar(&self) -> i32 {
    
        self.uart_in()

    }

    #[no_mangle]
    pub fn puts(&self, str: &str) -> i32 {

        for c in str.bytes() {
           self.putchar(c as i32);
        }
    
        0
    }

    #[no_mangle]
    pub fn puthex(&self, mut h: u32) {

        let mut cur_digit: i32;
        // Iterate through h taking top 4 bits each time and outputting ASCII of hex
        // digit for those 4 bits
        for _i in 0..8 {
            cur_digit = (h >> 28) as i32;
      
            if cur_digit < 10 {
                
                self.putchar((b'0' + (cur_digit as u8)) as i32);
            
            }
            else {
                
                self.putchar((b'A' - 10 + (cur_digit as u8)) as i32);
    
            }
      
          h <<= 4;
        
        }
    
    }

    #[no_mangle]
    pub fn putbyte(&self, h: u32) {
    
        let mut cur_digit: i32;
    
        cur_digit = (h >> 4) as i32;
        
        if cur_digit < 10 {
            
            self.putchar((b'0' + (cur_digit as u8)) as i32);
    
        }
        else {
    
            self.putchar((b'A' - 10 + (cur_digit as u8)) as i32);
    
        }
    
        cur_digit = (h & 0x0f) as i32;
        
        if cur_digit < 10 {
            
            self.putchar((b'0' + (cur_digit as u8)) as i32);
    
        }
        else {
    
            self.putchar((b'A' - 10 + (cur_digit as u8)) as i32);
    
        }
    
    }

    #[no_mangle]
    pub fn putdec(&self, n: u32) {

        if n > 9 {
            
            self.putdec(n/10);
    
        }
        
        self.putchar((b'0' + ((n % 10) as u8)) as i32);
    
    }


}