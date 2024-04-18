use core::arch::asm;


pub unsafe fn get_mepc() -> u32 {
    
    let result: u32;
    
    asm!("csrr {}, mepc", out(reg) result);

    return result

}

pub unsafe fn get_mcause() -> u32 {
    
    let result: u32;
    
    asm!("csrr {}, mcause", out(reg) result);

    return result

}

pub unsafe fn get_mtval() -> u32 {

    let result: u32;
    
    asm!("csrr {}, mtval", out(reg) result);

    return result

}

pub unsafe fn get_mcycle() -> u32 {

    let result: u32;
    
    asm!("csrr {}, mcycle", out(reg) result);

    return result


}

pub unsafe fn reset_mcycle() {

    asm!("csrw mcycle, x0")

}