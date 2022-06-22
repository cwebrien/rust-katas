#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(dim: u32) -> Rectangle {
        Rectangle {
            height: dim,
            width: dim,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        height: 30,
        width: 50,
    };
    let rect2: Rectangle = Rectangle::square(20);

    println!("rect1 is {:?} with area {}", rect1, rect1.area());
    println!("rect2 is {:?} with area {}", rect2, rect2.area());

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1? {}", rect2.can_hold(&rect1));
}
