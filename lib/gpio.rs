use core::ptr;

pub (crate) const GPIO_BASE: *mut u32 = 0x80000000 as *mut u32;

const GPIO_OUT_MASK : usize = 0xF;

pub struct Gpio {
    pub (crate) p: *mut u32
}

#[derive(Clone, Copy)]
enum OffsetGpioReg {
    OutReg = 0,
    InReg = 1,
    InDBNCReg = 2,
    OutShiftReg = 3,
}


impl Gpio {

    #[inline(always)]
    fn write_gpio(&self, offset : OffsetGpioReg, value: u32) {
    
        unsafe {
            
            ptr::write_volatile(&mut *(self.p.add(offset as usize)), value);
            
        }

    }

    #[inline(always)]
    fn read_gpio(&self, offset : OffsetGpioReg) -> u32 {

        unsafe {
            
            ptr::read_volatile(&mut *(self.p.add(offset as usize)))
            
        }
    
    }

    #[inline(always)]
    fn write_gpio_bit(&self, offset : OffsetGpioReg, value_bit_index: u32, mut value_bit: u32) {
        
        value_bit &= 1;

        let mut value_bits : u32 = self.read_gpio(offset);

        value_bits &= !(1 << value_bit_index);

        value_bits |= value_bit << value_bit_index;

        self.write_gpio(offset, value_bits);
    
    }

    #[inline(always)]
    fn read_gpio_bit(&self, offset : OffsetGpioReg, value_bit_index: u32) -> u32 {

        let mut value_bits : u32 = self.read_gpio(offset);

        value_bits >>= value_bit_index;

        value_bits &= 1;

        value_bits
    
    }

    pub fn set_output(&self, output : u32) {

        self.write_gpio(OffsetGpioReg::OutReg, output);

    }

    pub fn set_output_bit(&self, output_bit_index : u32, output_bit : u32) {
        
        self.write_gpio_bit(OffsetGpioReg::OutReg, output_bit_index, output_bit);
    
    }

    pub fn get_output_bit(&self, output_bit_index : u32) -> u32 {

        self.read_gpio_bit(OffsetGpioReg::OutReg, output_bit_index)

    }

}