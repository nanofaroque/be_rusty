use std::process;
use std::env;
fn main() {
    println!("My pid is {}", process::id());

   let key = "Omar";
   
   env::set_var(key, "Faroque");
   println!("{}",env::var(key).unwrap());
   assert_eq!(env::var(key), Ok("Faroque".to_string()));
   let key_global="Hello";   
   println!("{}",env::var(key_global).unwrap());
   loop{}
}
