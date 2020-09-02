fn main() {
    let x=5;
    let x=x+1; // each let create a new variable with the same name. first variable become shadow
    let x=x*2;
    println!("value of x:{} ",x)
    //x=x+10; it will show an error because variable are immutable by default
}