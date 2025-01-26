use emrust8::*;

fn main() {
    let mut hardware = Hardware::new();
    //println!("{:?}", hardware.memory); // debug print the memory
    // Todo read memory at program counter
    SI(&mut hardware, 2);
    SR(&mut hardware, 0, 0x2);
    SRS(&mut hardware, 5, 2);
    println!("GR:{:?}", hardware.general_registers);
    println!("{:?}", hardware.memory);

    //TODO maybe proc macro replacment?
    while hardware.display_buffer.window.is_open() && !hardware.display_buffer.should_close {
        //testing memory changes

        update_display_buffer(&mut hardware);
    }
}
