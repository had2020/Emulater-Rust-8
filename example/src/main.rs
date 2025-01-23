use emrust8::*;

fn main() {
    let mut hardware = Hardware::new();
    //println!("{:?}", hardware.memory); // debug print the memory

    //TODO maybe proc macro replacment?
    while hardware.display_buffer.window.is_open() && !hardware.display_buffer.should_close {
        let key = keyboard_to_string(&hardware);
        //println!("{}", key);

        decode(Opcode::ClearScreen);

        // process here
        update_display_buffer(&mut hardware);
    }
}
