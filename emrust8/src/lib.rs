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
    pub index_register: u16,        // stores memeory operations
    pub general_registers: Vec<u8>, // each index is a register
    pub delay_register: u8,
    pub sound_register: u8,
    pub program_Counter_register: u16, // stores currently executing
    pub stack_pointer_register: u8,    // point to topmost level of stack
    pub stack_array_register: Vec<u16>, //connected to stack_pointer_register
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

        // window and display buffer
        let (window_, buffer_) = defined_window(64, 32, "CHIP-8");

        Hardware {
            memory: zeros,
            index_register: index_register_,
            general_registers: general_registers_,
            delay_register: 0x00,
            sound_register: 0x00,
            program_Counter_register: 0x0000,
            stack_pointer_register: 0x00,
            stack_array_register: stack_array_register_,
            display_buffer: { window: window_, buffer_, },
        }
    }
}

pub struct display {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub should_close: bool,
    pub scaled_height: u8,
    pub scaled_width: u8,
    pub scale_factor: u8,
}
