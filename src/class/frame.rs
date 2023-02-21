use std::collections::VecDeque;

use crate::class::{Class, ConstantsPoolInfo, method::Descriptor};

pub struct Frame {
    pub class: Class,
    pub method: String,
    pub ip: u32,
    pub code: Vec<u8>,
    pub locals: Vec<i32>,
    pub stack: VecDeque<i32>,
}

impl Frame {
    pub fn exec(&mut self) -> i32 {
        //would rather not do JIT yet...
        loop {
            let op = self.code[self.ip as usize];
            println!(
                "method: {}, opcode: 0x{:x}, current stack:{:?}",
                self.method, op, self.stack
            );
            let n = self.stack.len();
            match op {
                0x2 => {
                    // iconst_m1
                    self.stack.push_back(-1);
                }
                0x3 => {
                    //iconst_0
                    self.stack.push_back(0);
                }
                0x4 => {
                    // iconst_1
                    self.stack.push_back(1);
                }
                0x5 => {
                    // iconst_2
                    self.stack.push_back(2);
                }
                0x6 => {
                    // iconst_3
                    self.stack.push_back(3);
                }
                0x7 => {
                    self.stack.push_back(4);
                }
                0x8 => {
                    self.stack.push_back(5);
                }
                0x15 => {
                    self.ip += 1;
                    self.stack.push_back(self.locals[self.code[self.ip as usize] as usize]);
                }
                0x1a => {
                    //iload_0
                    self.stack.push_back(self.locals[0]);
                }
                0x1b => {
                    // iload_1
                    self.stack.push_back(self.locals[1]);
                },
                0x1c => {
                    self.stack.push_back(self.locals[2]);
                },
                0x1d => {
                    self.stack.push_back(self.locals[3]);
                }
                0x36 => {
                    // istore     index
                    let value = self.stack.pop_front().unwrap();
                    self.ip += 1;
                    self.locals[self.code[self.ip as usize] as usize] = value;
                }
                0x3b => {
                    // istore_0
                    let value = self.stack.pop_front().unwrap();
                    self.locals[0] = value;
                }
                0x3c => {
                    let value = self.stack.pop_front().unwrap();
                    self.locals[1] = value;
                }
                0x3d => {
                    let value = self.stack.pop_front().unwrap();
                    self.locals[2] = value;
                }
                0x3e => {
                    let value = self.stack.pop_front().unwrap();
                    self.locals[3] = value;
                }
                0x60 => {
                    //iadd
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_add(a));
                }
                0x64 => {
                    // isub
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_sub(a));
                }
                0x68 => {
                    // imul
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_mul(a));
                }
                0x6c => {
                    //idiv
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_div(a));
                }
                0x84 => {
                    let index = self.code[(self.ip + 1) as usize];
                    let cons_t = self.code[(self.ip + 2) as usize];
                    self.locals[index as usize] += cons_t as i32;
                    self.ip += 2;
                }
                0xa7 => {
                    let branchbyte1 = self.code[(self.ip + 1) as usize];
                    println!("{}", branchbyte1);
                    let branchbyte2 = self.code[(self.ip + 2) as usize];
                    println!("{}", branchbyte2);
                    self.ip = self.ip.checked_add_signed((((((branchbyte1 as u16) << 8) | branchbyte2 as u16) - 1) as i16).into()).unwrap();
                    println!("{:?}", self.code);
                }
                0xac => {
                    // ireturn
                    let v = self.stack.pop_front().unwrap();
                    return v;
                }
                0xa2 => { // if_icmpge    brancbyte1      branchbyte2
                    let value1 = self.stack.pop_front().unwrap();
                    let value2 = self.stack.pop_front().unwrap();
                    let branchbyte1 = self.code[(self.ip + 1) as usize];
                    let branchbyte2 = self.code[(self.ip + 2) as usize];
                    if value1 >= value2 {
                        self.ip += (((branchbyte1 as u32) << 8) | branchbyte2 as u32) - 1;
                    } else {
                        self.ip += 2;
                    }
                }
                0xb8 => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    let method_info = &self.class.constant_pool[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1]; // i have to asssume that indices in terms of the internals of the jvm start at 1, otherwise i have no idea why i'd have to subtract 1 here.
                    if let ConstantsPoolInfo::MethodRef { class_index, name_and_type_index } = method_info {
                        let name_and_index = &self.class.constant_pool[*name_and_type_index as usize - 1];
                        if let ConstantsPoolInfo::NameAndType { name_index, descriptor_index } = name_and_index {
                            let desc = &self.class.constant_pool[*descriptor_index as usize - 1];
                            if let ConstantsPoolInfo::Utf8 { length, bytes } = desc {
                                let descriptor = Descriptor::new(bytes.clone());
                                let mut args: Vec<i32> = vec![];
                                for _ in descriptor.types {
                                    args.push(self.stack.pop_front().unwrap()); // will do type checking later.
                                }
                                let method = &self.class.constant_pool[*name_index as usize - 1];
                                if let ConstantsPoolInfo::Utf8 { length, bytes } = method {
                                    let mut frame = self.class.frame(bytes.clone(), &mut args);
                                    self.stack.push_back(frame.exec());
                                }
                            }
                        }
                    } else {
                        panic!("invokestatic was not a method reference!");
                    }
                    self.ip += 2;
                }
                _ => {
                    panic!("Unimplemented opcode: 0x{:x}", op);
                }
            }
            self.ip += 1;
        }
    }
}
