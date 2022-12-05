#![feature(proc_macro_hygiene)]
use std::io::{ Read, Write };
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use brainfuck_rust::ir;
use brainfuck_rust::opcode;

unsafe extern "sysv64" fn putchar(char: u8) {
    std::io::stdout().write_all(&[char]).unwrap()
}

/**
 * It works on CentOS-8 (Linux x86-64 asm)
 */
#[derive(Default)]
struct Interpreter;

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let ir_code = ir::Code::from(opcode_code.instructions)?;
        let mut loops = vec![];

        let mut ops = dynasmrt::x64::Assembler::new()?;
        let entry_point = ops.offset();

        dynasm!(ops
            ; .arch x64
            ; mov rcx, rdi
        );

        for ir in ir_code.instructions {
            match ir {
                ir::IR::SHL(x) => dynasm!(ops
                    ; sub rcx, x as i32 // sp -= x
                ),
                ir::IR::SHR(x) => dynasm!(ops
                    ; add rcx, x as i32 // sp += x
                ),
                ir::IR::ADD(x) => dynasm!(ops
                    ; add BYTE [rcx], x as i8 // *sp += x
                ),
                ir::IR::SUB(x) => dynasm!(ops
                    ; sub BYTE [rcx], x as i8 // *sp -= x
                ),
                ir::IR::PUTCHAR => dynasm!(ops
                    ; mov r12, rcx
                    ; mov rdi, [rcx]
                    ; mov rax, QWORD putchar as _
                    ; sub rsp, BYTE 0x28
                    ; call rax
                    ; add rsp, BYTE 0x28
                    ; mov rcx, r12
                ),
                ir::IR::GETCHAR => {}
                ir::IR::JIZ(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    loops.push((l, r));
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jz => r
                        ; => l
                    )
                }
                ir::IR::JNZ(_) => {
                    let (l, r) = loops.pop().unwrap();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => l
                        ; => r
                    )
                }
            }
        }

        dynasm!(ops
            ; ret
        );

        let exec_buffer = ops.finalize().unwrap();
        let mut memory: Box<[u8]> = vec![0; 65536].into_boxed_slice();
        let memory_addr_from = memory.as_mut_ptr();
        let memory_addr_to = unsafe { memory_addr_from.add(memory.len()) };
        let func: fn(memory_addr_from: *mut u8, memory_addr_to: *mut u8) =
            unsafe { std::mem::transmute(exec_buffer.ptr(entry_point)) };
        func(memory_addr_from, memory_addr_to);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let mut file = std::fs::File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut interpreter = Interpreter::default();
    interpreter.run(data)
}