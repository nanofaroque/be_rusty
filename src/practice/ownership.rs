fn main() {
    let s  = String::from("hello world");
    take_ownership(s);

    // println!("{}",s) it will throw an error because take_owner function removed
    // the data from the heap

    let s1  = String::from("hello world");
    take_ownership_another(&s1);
    println!("{}",s1)


}
fn take_ownership(some_string:String){
    println!("{}", some_string);
}

fn take_ownership_another(some_string: &str){
    println!("{}", some_string);
}