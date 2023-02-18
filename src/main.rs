use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use bytes::Bytes;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let f = File::open(query).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let mut class = vastatrix::class::Class::new(Bytes::from(buffer));
    println!("{:?}", class);
    let mut add_frame = class.frame("add".to_string(), &mut vec![10, 5]);
    let mut sub_frame = class.frame("sub".to_string(), &mut vec![5, 3]);
    let mut mul_frame = class.frame("mul".to_string(), &mut vec![6, 4]);
    let mut div_frame = class.frame("div".to_string(), &mut vec![10, 2]);
    let mut fib_frame = class.frame("fib".to_string(), &mut vec![12]);
    let add_result = add_frame.exec();
    let sub_result = sub_frame.exec();
    let mul_result = mul_frame.exec();
    let div_result = div_frame.exec();
    let fib_result = fib_frame.exec();
    println!("ADD RESULT!: {}", add_result);
    println!("SUB RESULT!: {}", sub_result);
    println!("MUL RESULT!: {}", mul_result);
    println!("DIV RESULT!: {}", div_result);
    println!("FIB RESULT!: {}", fib_result);
}
