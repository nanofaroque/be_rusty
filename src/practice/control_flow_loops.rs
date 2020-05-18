fn main() {
    let mut count =0;
    loop{
        println!("Hello world");
        if count==10{
            break;
        }
        count+=1;
    }

    // Returning value from the loops
    let result = loop{
        println!("Hello world");
        if count==10{
            break count*2;
        }
        count+=1;
    };
    println!("{}",result)
}