//! Helpers for reading and uploading executables into the ROM

use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use crate::cpu::Cpu;

fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open the binary file");
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    contents
}

fn upload_to_rom(cpu: &mut Cpu, content: Vec<u8>) {

    let mut address = 0x8000;

    for byte in content {
        cpu.memory.write_byte(address, byte);
        address += 1;
    }

}