use core::ptr;

use core::arch::asm;

extern "C" {
    static _vectors_start: u32;
    
}

#[no_mangle]
pub unsafe fn install_exception_handler(vector_num : u32, handler_fn : extern "riscv-interrupt-m" fn()) -> u32 {

    let exc_vectors: *const u32 = &_vectors_start;

    if vector_num >= 32 {
        return 1
    }
    
    let handler_jmp_loc = exc_vectors.add(vector_num as usize);
    
    let offset : i32 = (handler_fn as u32).wrapping_sub(handler_jmp_loc as u32) as i32;
    
    if (offset  >= (1 << 19)) || (offset  < -(1 << 19)) {
        return 2
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
            
    return 0

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