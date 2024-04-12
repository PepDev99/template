use core::ptr;

pub const GPIO_OUT_REG : usize =  0x0;
pub const GPIO_IN_REG : usize =  0x4;
pub const GPIO_IN_DBNC_REG : usize =  0x8;
pub const GPIO_OUT_SHIFT_REG : usize = 0xC;

const GPIO_OUT_MASK : usize = 0xF;

pub type Gpio = *mut u32;

pub fn set_outputs(gpio: Gpio, outputs: u32) {
    
    unsafe {
        ptr::write_volatile(gpio, outputs);
    }

}

pub fn read_gpio(gpio: Gpio) -> u32 {

    unsafe {
        return ptr::read_volatile(gpio);
    }
    
}

pub fn set_output_bit(gpio: Gpio, output_bit_index: u32, mut output_bit: u32) {

    output_bit &= 1;

    unsafe {

        let mut output_bits : u32 = ptr::read_volatile(gpio);

        output_bits &= !(1 << output_bit_index);

        output_bits |= output_bit << output_bit_index;

        set_outputs(gpio, output_bits);
    }

}

pub fn get_output_bit(gpio: Gpio, output_bit_index: u32) -> u32 {

    unsafe {

        let mut output_bits : u32 = ptr::read_volatile(gpio);

        output_bits >>= output_bit_index;

        output_bits &= 1;

        return output_bits;

    }

}