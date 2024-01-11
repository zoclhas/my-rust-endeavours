#[derive(Debug)]
struct Rectanlge {
    width: u32,
    height: u32,
}

impl Rectanlge {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectanlge) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectanlge {
    fn square(size: u32) -> Rectanlge {
        Rectanlge {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectanlge {
        width: 250,
        height: 129,
    };
    let rect1 = Rectanlge {
        width: 25,
        height: 12,
    };
    let rect2 = Rectanlge::square(32);

    println!("Can rect1 hold rect? {}", rect1.can_hold(&rect));
    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("The area of rectangle is {} square units.", rect.area());
    println!("Square of 32 units: {:#?}", rect2)
}

