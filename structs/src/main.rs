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

fn main() {
    let rect = Rectanlge {
        width: 250,
        height: 129,
    };
    let rect1 = Rectanlge {
        width: 25,
        height: 12,
    };

    println!("Can rect1 hold rect? {}", rect1.can_hold(&rect));
    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("The area of rectangle is {} square units.", rect.area());
}

