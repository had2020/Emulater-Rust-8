use minifb::{Key, Window, WindowOptions};

pub fn defined_window(width: usize, height: usize, name: &str) -> (Window, Vec<u32>) {
    // initialize the pixel buffer
    let buffer: Vec<u32> = vec![0; width * height];

    // create a window
    let window = Window::new(
        name,
        width,
        height,
        //windowOptions::default(),
        WindowOptions {
            resize: false,
            borderless: false,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e);
    });

    (window, buffer)
}

// virtual emulater based of the CHIP-8
pub struct Hardware {
    pub memory: Vec<u8>, // each index is a byte in memory

    // registers
    pub index_register: u16,           // stores memeory operations
    pub general_registers: Vec<u8>,    // each index is a register, accessable for programs
    pub delay_register: u8,            // decressed every 60 hz
    pub sound_register: u8,            // TODO!
    pub program_Counter_register: u16, // stores currently executing, just one address jumps by twos aka 0x02
    pub stack_pointer_register: u8,    // point to topmost level of stack, one number/ byte
    pub stack_register: Vec<u16>,      //connected to stack_pointer_register, even jumps
    pub display_buffer: display,
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

        // creating empty 16 stack_array_register
        let mut stack_array_register_: Vec<u16> = Vec::new();
        for byte in 0..16 {
            stack_array_register_.push(0x0000);
        }

        // window and display buffer, scaled verison of 64 and 32
        let (window_, buffer_) = defined_window(640, 320, "CHIP-8");

        Hardware {
            memory: zeros,
            index_register: index_register_,
            general_registers: general_registers_,
            delay_register: 0x00,
            sound_register: 0x00,
            program_Counter_register: 0x200, // fake allocation. Why not program_interater?
            stack_pointer_register: 0x00,
            stack_register: stack_array_register_,
            display_buffer: display {
                window: window_,
                buffer: buffer_,
                should_close: false,
            },
        }
    }
}

pub struct display {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub should_close: bool,
}

// 6-key hexadecimal keypad:
/*
1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F
*/
pub fn keyboard_to_string(hardware: &Hardware) -> String {
    let mut key_pressed: &str = "";
    if let Some(key) = hardware.display_buffer.window.get_keys().iter().next() {
        match key {
            Key::Key1 => key_pressed = "1",
            Key::Key2 => key_pressed = "2",
            Key::Key3 => key_pressed = "3",
            Key::C => key_pressed = "C",
            Key::Key4 => key_pressed = "4",
            Key::Key5 => key_pressed = "5",
            Key::Key6 => key_pressed = "6",
            Key::D => key_pressed = "D",
            Key::Key7 => key_pressed = "7",
            Key::Key8 => key_pressed = "8",
            Key::Key9 => key_pressed = "9",
            Key::E => key_pressed = "E",
            Key::A => key_pressed = "A",
            Key::Key0 => key_pressed = "0",
            Key::B => key_pressed = "B",
            Key::F => key_pressed = "F",
            _ => key_pressed = "",
        }
    }
    key_pressed.to_string()
}

//TODO sprite size of 8x15, for keyboard letters

pub fn update_display_buffer(hardware: &mut Hardware) {
    hardware
        .display_buffer
        .window
        .update_with_buffer(&hardware.display_buffer.buffer, 640, 320)
        .unwrap();
}

// TODO intruction set functions
pub struct IntructionSet {
    pub set: Vec<String>,
}

// TODO Opcode Enum replacement?

// Clear the display.
pub fn CLS(hardware: &mut Hardware) {
    hardware.display_buffer.buffer.fill(0);
    hardware.program_Counter_register += 0x02;
}

// Return from a subroutine.
pub fn RET() {}

//Jump to a machine code routine at nnn.
pub fn JP(hardware: &mut Hardware, addr: u16) {
    hardware.program_Counter_register = addr;
}

pub fn CALL(hardware: &mut Hardware, addr: u16) {
    hardware.stack_pointer_register += 1;
    hardware.program_Counter_register = addr; // pc set to nnn
    hardware.stack_register[hardware.stack_pointer_register as usize] =
        hardware.program_Counter_register;
    //program_Counter_register = last stack index plus change? line 2 TODO remove this comment once tested
}

// basicly, compare if equal to jump 4 instead of 2
pub fn SE(hardware: &mut Hardware, Vx: u8, KK: u8) {
    if Vx == KK {
        hardware.program_Counter_register += 0x04;
    } else {
        hardware.program_Counter_register += 0x02;
    }
}

// not if
pub fn SNE(hardware: &mut Hardware, Vx: u8, KK: u8) {
    if Vx != KK {
        hardware.program_Counter_register += 0x04;
    } else {
        hardware.program_Counter_register += 0x02;
    }
}

// compares together both of the register's data
pub fn SRE(hardware: &mut Hardware, Vx: u8, Vy: u8) {
    if Vx == Vy {
        hardware.program_Counter_register += 0x04;
    } else {
        hardware.program_Counter_register += 0x02;
    }
}

// the interpreter puts the value kk into register index Vx
pub fn SR(hardware: &mut Hardware, register_Index_num_Vx: usize, KK: u8) {
    hardware.general_registers[register_Index_num_Vx] = KK;
    hardware.program_Counter_register += 0x02;
}

// adds the value kk to the value of register Vx then stores the result in Vx
pub fn ADD(hardware: &mut Hardware, register_Index_num_Vx: usize, KK: u8) {
    hardware.general_registers[register_Index_num_Vx] += KK;
    hardware.program_Counter_register += 0x02;
}

// stores the value of register Vy in register Vx
pub fn SRTR(hardware: &mut Hardware, register_Index_num_Vx: usize, register_Index_num_Vy: usize) {
    hardware.general_registers[register_Index_num_Vx] =
        hardware.general_registers[register_Index_num_Vy];
    hardware.program_Counter_register += 0x02;
}

// set Vx = Vx OR Vy, limted to registers
pub fn OR(hardware: &mut Hardware, register_Index_num_Vx: usize, register_Index_num_Vy: usize) {
    let first: u8 = hardware.general_registers[register_Index_num_Vx];
    let second: u8 = hardware.general_registers[register_Index_num_Vy];

    let mut first_inter: u8 = 0;
    let mut second_inter: u8 = 0;

    let mut current_first_byte: u8 = 0;
    let mut current_second_byte: u8 = 0;

    let mut or_sum: String = String::new();

    for bit in 0..first {
        first_inter += 1;
        current_first_byte = bit;
        for bitY in 0..second {
            second_inter += 1;
            current_second_byte = bitY;
            // right number_place
            if second_inter == first_inter {
                if current_first_byte == 1 && current_second_byte == 1 {
                    or_sum.push('1');
                } else if current_first_byte == 1 && current_second_byte == 0 |  {

                }
            }
        }
    }

    hardware.general_registers[register_Index_num_Vx] = or_sum;
    hardware.program_Counter_register += 0x02;
}

//TODO
#[derive(Debug)]
pub enum Opcode {
    ClearScreen,                                     // 00E0
    Return,                                          // 00EE
    Jump { address: u16 },                           // 1NNN
    Call { address: u16 },                           // 2NNN
    SkipEqual { register: u8, value: u8 },           // 3XNN
    SkipNotEqual { register: u8, value: u8 },        // 4XNN
    SkipRegEqual { reg_x: u8, reg_y: u8 },           // 5XY0
    SetRegister { register: u8, value: u8 },         // 6XNN
    AddToRegister { register: u8, value: u8 },       // 7XNN
    SetRegisterToReg { reg_x: u8, reg_y: u8 },       // 8XY0
    Or { reg_x: u8, reg_y: u8 },                     // 8XY1
    And { reg_x: u8, reg_y: u8 },                    // 8XY2
    Xor { reg_x: u8, reg_y: u8 },                    // 8XY3
    AddRegisters { reg_x: u8, reg_y: u8 },           // 8XY4
    SubRegisters { reg_x: u8, reg_y: u8 },           // 8XY5
    ShiftRight { register: u8 },                     // 8XY6
    SubN { reg_x: u8, reg_y: u8 },                   // 8XY7
    ShiftLeft { register: u8 },                      // 8XYE
    SkipNotRegEqual { reg_x: u8, reg_y: u8 },        // 9XY0
    SetIndex { address: u16 },                       // ANNN
    JumpWithOffset { address: u16 },                 // BNNN
    Random { register: u8, mask: u8 },               // CXNN
    DrawSprite { reg_x: u8, reg_y: u8, height: u8 }, // DXYN
    SkipIfKeyPressed { register: u8 },               // EX9E
    SkipIfKeyNotPressed { register: u8 },            // EXA1
    GetDelayTimer { register: u8 },                  // FX07
    WaitKeyPress { register: u8 },                   // FX0A
    SetDelayTimer { register: u8 },                  // FX15
    SetSoundTimer { register: u8 },                  // FX18
    AddToIndex { register: u8 },                     // FX1E
    SetIndexToSprite { register: u8 },               // FX29
    StoreBCD { register: u8 },                       // FX33
    StoreRegisters { register: u8 },                 // FX55
    LoadRegisters { register: u8 },                  // FX65
}

/*
pub fn decode(hardware: &mut Hardware, opcode: Opcode) {
    match opcode {
        Opcode::ClearScreen => {
            hardware.display_buffer.buffer.fill(0);
        }
        _ => println!("ew"),
    }
}
*/
