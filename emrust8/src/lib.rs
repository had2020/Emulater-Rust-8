// virtual emulater based of the CHIP-8
pub struct Hardware {
    pub memory: Vec<u8>,    // each index is a byte in memory

    // registers
    pub index_register: u16 // stores memeory operations
    pub general_registers: Vec<u8>, // each index is a register
    pub delay_sound_timers_register: Vec<u8> // 2 max TODO
    pub program_Counter_register: u16 // stores currently executing
    pub stack_pointer_register: u8 // point to topmost level of stack
    // TODO interpecter and The Stack
    // 16 return addresses (16-bit each
    // connected to stack_pointer_register
}

// TODO display with minifb
impl Hardware {
    pub fn new() -> Hardware {
        // creates 4 kilobytes, of 0x00 bytes
        let mut zeros: Vec<u8> = Vec::new();
        // 512 of the first bytes are reserved for the system
        for byte in 0..4096 {
            //TODO interpreter, and it's allocation
            // their are 4 kilobytes of memory
            zeros.push(0x00); // hexadecimal
        }

        // 16, 8bit, going from V0-9 and VA-VF
        let mut general_registers_: Vec<u8> = Vec::new();

        // 0x0000 - 0xFFFF, memory operations
        let index_register_: u16 = 0x0000;
        Hardware { memory: zeros, index_register: index_register_,  general_registers: general_registers_}
    }
}

// TODO 16 general purpose 8-bit registers
