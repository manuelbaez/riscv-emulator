use std::{env, fs::File, io::Read, time::Instant};

use consts::DRAM_SIZE;
use cpu::Cpu;

#[cfg(feature = "debug")]
use std::{thread, time::Duration};

use crate::cpu::side_effects::OperationSideEffect;
#[cfg(feature = "debug")]
use crate::debug::{init_debug_print_thread_channel, DebugMessages};

mod consts;
mod cpu;
#[cfg(feature = "debug")]
mod debug;
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

    #[cfg(feature = "debug")]
    let (debug_thread_handle, debug_tx) = init_debug_print_thread_channel();

    let mut cpu = Cpu::new(DRAM_SIZE, code);

    let now = Instant::now();
    loop {
        #[cfg(feature = "debug")]
        let debug_cycle_start = now.elapsed().as_nanos();

        let instruction = match cpu.fetch_next_instruction() {
            Ok(inst) => inst,
            Err(err) => {
                let program_counter = cpu.get_program_counter();
                eprintln!("{program_counter:0x}: {err}");
                break;
            }
        };
        
        // #[cfg(feature = "debug")]
        let fetched_pc = cpu.get_program_counter();
        let execution_result = cpu.execute(instruction);

        #[cfg(feature = "debug")]
        {
            debug_tx
                .send(DebugMessages::InstructionExecution {
                    time_elapsed_ns: now.elapsed().as_nanos() - debug_cycle_start,
                    pc: fetched_pc,
                    instruction,
                    registers: cpu.get_registers(),
                })
                .unwrap();
        }

        match execution_result {
            Ok(OperationSideEffect::None) => (),
            Ok(OperationSideEffect::SkipPCIncrease) => {
                continue;
            }
            Err(err) => {
                eprintln!("{fetched_pc:0x}: {instruction:0x} {err}");
                break;
            }
        };
        cpu.increase_program_counter();
    }

    let run_time = now.elapsed();
    #[cfg(feature = "debug")]
    {
        debug_tx
            .send(DebugMessages::DumpRegisters(cpu.get_registers()))
            .unwrap();
        debug_tx.send(DebugMessages::Terminate).unwrap();
        loop {
            if debug_thread_handle.is_finished() {
                break;
            };
            thread::sleep(Duration::from_millis(100));
        }
    }
    println!("Total Execution time: {:.2?}", run_time);
}
