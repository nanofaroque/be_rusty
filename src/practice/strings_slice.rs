fn main() {
    let s = String::from("hello world"); // this string is mutable and stored in heap

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}",hello);
    println!("{}",world);
}