use crate::class::Class;

pub struct Frame {
    pub class: Class,
    pub ip: u32,
    pub code: Vec<u8>,
    pub locals: Vec<u8>,
    pub stack: Vec<u8>,
}

impl Frame {
    pub fn exec(&mut self) -> u8 {
        loop {
            let op = self.code[self.ip as usize];
            println!("opcode: 0x{:x}, current stack:{:?}", op, self.stack);
            let n = self.stack.len();
            match op {
                26 => { //iload_0
                    self.stack.push(self.locals[0]);
                },
                27 => { // iload_1
                    self.stack.push(self.locals[1]);
                },
                96 => { //iadd
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = a + b;
                    self.stack = self.stack[..n-1].to_vec();
                }
                172 => { // ireturn
                    let v = self.stack[n - 1];
                    self.stack = self.stack[..n-1].to_vec();
                    return v;
                }
                _ => {}
            }
            self.ip += 1;
        }
    }
}