fn halves_are_alike(s: &String) -> bool {
    let a = &s[..s.len() / 2];
    let b = &s[s.len() / 2..];

    let vowels = "aeiouaeiou";
    let mut a_count = 0;
    let mut b_count = 0;

    for el in a.chars() {
        if vowels.contains(el) {
            a_count += 1
        }
    }
    for el in b.chars() {
        if vowels.contains(el) {
            b_count += 1
        }
    }

    a_count == b_count
}

fn main() {
    println!("Hello, world!");
    let some = String::from("hi");
    let res = halves_are_alike(&some);
    println!("{res}")
}
