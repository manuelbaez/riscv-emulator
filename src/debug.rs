use std::{
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
};

pub enum DebugMessages {
    InstructionExecution {
        time_elapsed_ns: u128,
        pc: u64,
        instruction: u32,
        registers: [u64; 32],
    },
    DumpRegisters([u64; 32]),
    Terminate,
}

pub fn init_debug_print_thread_channel() -> (JoinHandle<()>, Sender<DebugMessages>) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || loop {
        let value: DebugMessages = rx.recv().expect("Unable to receive from channel");
        match value {
            DebugMessages::InstructionExecution {
                time_elapsed_ns,
                pc,
                instruction,
                registers,
            } => {
                let sp = registers[2];
                println!("{pc:0x}: {instruction:06x} - {time_elapsed_ns}ns ");
            }
            DebugMessages::DumpRegisters(registers) => dump_registers(registers),
            DebugMessages::Terminate => break,
        }
    });

    (handle, tx)
}

// TODO: Later reimplement this, copied from RISC-V emulator rust book
fn dump_registers(registers: [u64; 32]) {
    let mut output = String::from("");
    let abi = [
        "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
        " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
        " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
    ];
    for i in (0..32).step_by(4) {
        output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x}\tx{:02}({})={:>#18x}\tx{:02}({})={:>#18x}\tx{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    registers[i],
                    i + 1,
                    abi[i + 1],
                    registers[i + 1],
                    i + 2,
                    abi[i + 2],
                    registers[i + 2],
                    i + 3,
                    abi[i + 3],
                    registers[i + 3],
                )
            );
    }
    println!("{}", output);
}
