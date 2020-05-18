fn main() {
    let x = helloworld();
    println!("{}",x)
}

fn helloworld()->String{
    let output= concat!("hello world ","faroque");
    return  output.to_string();

}