#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);


    let point: Point = Point { x: 0.4, y: 0.4 };
    println!("point coordinates: ({},{})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("point coordinates bottom_right: ({},{})", bottom_right.x, bottom_right.y);
}
