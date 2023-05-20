#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 20,
        height: 60
    };

    dbg!(&rect1);

    println!("Area of rectangle {:?} is {}", rect1, rect1.area());
    println!("{:?} can hold {:?}? {}", rect1, rect2, rect1.can_hold(&rect2));
}
