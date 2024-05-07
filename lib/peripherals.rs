#[path = "./uart.rs" ] mod uart;
#[path = "./gpio.rs" ] mod gpio;
#[path = "./timer.rs" ] mod timer;

use core::ptr::replace;
pub struct Peripherals {
    serial: Option<uart::Uart>,
    gpio: Option<gpio::Gpio>,
    timer: Option<timer::Timer>,
}

impl Peripherals {
    
    #[no_mangle]
    pub unsafe fn take_serial(&mut self) -> uart::Uart {

        let p = replace(&mut self.serial, None);
        p.unwrap()

    }

    #[no_mangle]
    pub unsafe fn take_gpio(&mut self) -> gpio::Gpio {

        let p = replace(&mut self.gpio, None);
        p.unwrap()
    }

    #[no_mangle]
    pub unsafe fn take_timer(&mut self) -> timer::Timer {

        let p = replace(&mut self.timer, None);
        p.unwrap()
    }

}

pub static mut PERIPHERALS: Peripherals = Peripherals {
    serial: Some(uart::Uart{p: uart::UART0_BASE}),
    gpio : Some(gpio::Gpio{p: gpio::GPIO_BASE}),
    timer: Some(timer::Timer{p: timer::TIMER_BASE}),
};