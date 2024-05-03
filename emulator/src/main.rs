use std::{env, fs::File, io::Read};

use consts::DRAM_SIZE;
use cpu::Cpu;

mod consts;
mod cpu;
mod error;
mod memory;
mod system_bus;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: emulator <filename>");
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut code = Vec::new();
    file.read_to_end(&mut code).unwrap();

    let mut cpu = Cpu::new(DRAM_SIZE, code);

    loop {
        let instruction = match cpu.fetch_next_instruction() {
            Ok(inst) => inst,
            Err(_) => break,
        };
        match cpu.execute(instruction) {
            Ok(_) => (),
            Err(err) => {
                dbg!(err);
                break;
            }
        };
    }
    cpu.dump_registers();
}
