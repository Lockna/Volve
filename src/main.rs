use std::env;
use volve::{code, cpu::Cpu};

fn main() {
    let binary = code::read_file(env::args().nth(1).unwrap());

    let mut cpu = Cpu::new();

    code::upload_to_rom(&mut cpu, binary);

    cpu.run();
}
