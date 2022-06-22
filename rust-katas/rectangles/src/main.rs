#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn main() {
    let rect: Rectangle = Rectangle{height: 30, width: 50};

    println!("rect is {:?} with area {}", rect, area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
