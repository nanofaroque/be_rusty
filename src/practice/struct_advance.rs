fn main() {
    let user1 = Person {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = Person {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{:?}", user2);
}

#[derive(Debug)]
struct Person {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}