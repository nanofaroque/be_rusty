fn main() {
    let mut p = Person {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    p.email = String::from("myemail@email.com");
    println!("{}", p.active);
    println!("{}", p.username);
    println!("{}", p.sign_in_count);
    println!("{}", p.email);
}

struct Person {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}