use emrust8::*;

fn main() {
    let mut hardware = Hardware::new();
    println!("{:?}", hardware.memory);

    //TODO maybe proc macro replacment?
    while hardware.window.is_open() && !hardware.should_close {
        // process here
        hardware
            .window
            .update_with_buffer(&hardware.buffer, 640, 320)
            .unwrap();
    }
}
