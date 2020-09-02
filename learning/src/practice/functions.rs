fn main() {
    hello();
}

fn hello(){
    let msg="ping pong";// this msg string is type of &str and it is different that String
    // to use it with the world function, we have to convert into String type by using to_string();
    let msg = "ofaroque".to_string();
    world(msg)
}

fn world(name :String){
    println!("Hello world, {}",name)
}