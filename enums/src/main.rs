fn main() {
    let some_value = Some(3);

    // If some_value mathces Some(3) then do stuff
    if let Some(3) = some_value {
        println!("Three");
    }
}
