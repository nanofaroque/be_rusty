use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::str;

fn main() {
    let args: Vec<String>= env::args().collect();
    let input = &args[1];
    let output = &args[2];
    println!("{},{}",input,output);

    let mut cont:bool=true;

    let mut inputFile= File::open(input).unwrap();
    let mut buf=[0u8;12];// read 12 byte by 12 byte
    let mut buffer = File::create(output).unwrap();


    while cont {
        let readBytes= inputFile.read(&mut buf).unwrap();
        if readBytes!=buf.len(){
            cont=false
        }
        println!("{:?}",buf);
       buffer.write_all(buf.as_ref());
    }
    buffer.flush().unwrap();
}
