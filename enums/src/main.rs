fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0); // Try to get the value from y, or else use the default value specified in the function
    println!("{}", sum);
}
