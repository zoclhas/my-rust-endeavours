fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // If x matches the code below, execute it
        Some(i) => Some(i + 1),
        // If any other pattern, do this
        _ => None,
    }
}
