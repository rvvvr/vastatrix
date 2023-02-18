use crate::class::Class;

pub struct Frame {
    pub class: Class,
    pub method: String,
    pub ip: u32,
    pub code: Vec<u8>,
    pub locals: Vec<i32>,
    pub stack: Vec<i32>,
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
                    let mut thing = vec![-1];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x3 => {
                    //iconst_0
                    let mut thing = vec![0];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x4 => {
                    // iconst_1
                    let mut thing = vec![1];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x5 => {
                    // iconst_2
                    let mut thing = vec![2];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x6 => {
                    // iconst_3
                    let mut thing = vec![3];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x7 => {
                    let mut thing = vec![4];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x8 => {
                    let mut thing = vec![5];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x15 => {
                    self.ip += 1;
                    let mut thing = vec![self.locals[self.code[self.ip as usize] as usize]];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x1a => {
                    //iload_0
                    let mut thing = vec![self.locals[0]];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x1b => {
                    // iload_1
                    let mut thing = vec![self.locals[1]];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                },
                0x1c => {
                    let mut thing = vec![self.locals[2]];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                },
                0x1d => {
                    let mut thing = vec![self.locals[3]];
                    thing.append(&mut self.stack);
                    self.stack = thing;
                }
                0x36 => {
                    // istore     index
                    let value = self.stack.pop().unwrap();
                    self.ip += 1;
                    self.locals[self.code[self.ip as usize] as usize] = value;
                }
                0x3b => {
                    // istore_0
                    let value = self.stack.pop().unwrap();
                    self.locals[0] = value;
                }
                0x3c => {
                    let value = self.stack.pop().unwrap();
                    self.locals[1] = value;
                }
                0x3d => {
                    let value = self.stack.pop().unwrap();
                    self.locals[2] = value;
                }
                0x3e => {
                    let value = self.stack.pop().unwrap();
                    self.locals[3] = value;
                }
                0x60 => {
                    //iadd
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_add(a);
                    self.stack = self.stack[..n - 1].to_vec();
                }
                0x64 => {
                    // isub
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_sub(a);
                    self.stack = self.stack[..n - 1].to_vec();
                }
                0x68 => {
                    // imul
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_mul(a);
                    self.stack = self.stack[..n - 1].to_vec();
                }
                0x6c => {
                    //idiv
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_div(a);
                    self.stack = self.stack[..n - 1].to_vec();
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
                    let v = self.stack[n - 1];
                    self.stack = self.stack[..n - 1].to_vec();
                    return v;
                }
                0xa2 => { // if_icmpge    brancbyte1      branchbyte2
                    let value1 = self.stack.pop().unwrap();
                    let value2 = self.stack.pop().unwrap();
                    let branchbyte1 = self.code[(self.ip + 1) as usize];
                    let branchbyte2 = self.code[(self.ip + 2) as usize];
                    if value1 >= value2 {
                        self.ip += (((branchbyte1 as u32) << 8) | branchbyte2 as u32) - 1;
                    } else {
                        self.ip += 2;
                    }
                }
                _ => {
                    panic!("Unimplemented opcode: 0x{:x}", op);
                }
            }
            self.ip += 1;
        }
    }
}
