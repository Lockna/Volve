//! Helpers for reading and uploading executables into the ROM

use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::cpu::Cpu;

pub fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open the binary file");
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    contents
}

pub fn upload_to_rom(cpu: &mut Cpu, content: Vec<u8>) {

    let mut address = 0x7FFF;

    for byte in content {
        address += 1;
        cpu.memory.write_byte(address, byte);
    }

}