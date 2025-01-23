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

        // window and display buffer, scaled verison of 64 and 32
        let (window_, buffer_) = defined_window(640, 320, "CHIP-8");

        Hardware {
            memory: zeros,
            index_register: index_register_,
            general_registers: general_registers_,
            delay_register: 0x00,
            sound_register: 0x00,
            program_Counter_register: 0x0000,
            stack_pointer_register: 0x00,
            stack_array_register: stack_array_register_,
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

// Opcode Enum replacement?
pub fn CLS(hardware: &mut Hardware) {
    hardware.display_buffer.buffer.fill(0);
}

pub fn RET() {}
