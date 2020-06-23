fn give_commoner(gift: Option<&str>){
    println!("{}", "rusy");
    // This explicit handling giving more precise control
    match gift {
        Some("snake") =>{
            println!("{}", "It is a snake");
        },
        Some(_inner) =>{
          println!("{}","inner has been covered");
        },
        None => {
            println!("{}", "It is a NONE");
        }
    }
}
// this is better if you wants panic
fn give_princess(gift: Option<&str>){
    let inside=gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }
    println!("I love {}s!!!!!", inside);
}
fn main() {
    println!("{}", "rusy");
    give_commoner(Some("snake"));
    give_commoner(None);

    give_princess(Some("snake"));
    give_princess(None);
}
