fn greet() {
    println!("Hi.");
}

fn four() -> i32 {
    return 4;
}

fn main() {
    greet();
    println!("{}", four());
}