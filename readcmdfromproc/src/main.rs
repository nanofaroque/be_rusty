use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

fn main() {
    println!("My pid is {}", process::id());    
    let mut path="/proc/".to_string();
    let id:String=process::id().to_string();

    path=path+&id+"/cmdline";   

 let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();    
    println!("{}",contents);
}
