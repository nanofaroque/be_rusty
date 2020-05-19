fn main() {
    let rect = Rectangle{
        width: 12,
        height: 13
    };
    println!("{}",area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

struct Rectangle {
    width: u32,
    height: u32,
}