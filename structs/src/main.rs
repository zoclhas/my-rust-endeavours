struct Rectanlge {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectanlge {
        width: 50,
        height: 100,
    };

    println!("The area of rectangle is {} square units.", area(&rect));
}

fn area(rect: &Rectanlge) -> u32 {
    rect.width * rect.height
}

