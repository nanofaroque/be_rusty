fn main() {
    let x=-1;
    if x>0{
        println!("{}",x);
    }else{
        println!("{}",x-1);
    }


    // if in a let statement
    let m=-10;
    let num=if m>0 {5} else {6};
    println!("{}",num)
}