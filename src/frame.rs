use crate::class::Class;

pub struct Frame {
    pub class: Class,
    pub method: String,
    pub ip: u32,
    pub code: Vec<u8>,
    pub locals: Vec<u8>,
    pub stack: Vec<u8>,
}

impl Frame {
    pub fn exec(&mut self) -> u8 { //would rather not do JIT yet...
        loop {
            let op = self.code[self.ip as usize];
            println!("method: {}, opcode: 0x{:x}, current stack:{:?}", self.method, op, self.stack);
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
                    self.stack[n - 2] = b.wrapping_add(a);
                    self.stack = self.stack[..n-1].to_vec();
                },
                100 => { // isub
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_sub(a);
                    self.stack = self.stack[..n-1].to_vec();
                },
                104 => { // imul
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_mul(a);
                    self.stack = self.stack[..n-1].to_vec();
                },
                108 => { //idiv
                    let a = self.stack[n - 1];
                    let b = self.stack[n - 2];
                    self.stack[n - 2] = b.wrapping_div(a);
                    self.stack = self.stack[..n-1].to_vec();
                },
                172 => { // ireturn
                    let v = self.stack[n - 1];
                    self.stack = self.stack[..n-1].to_vec();
                    return v;
                }
                _ => {
                    panic!("Unimplemented opcode: 0x{:x}", op);
                }
            }
            self.ip += 1;
        }
    }
}